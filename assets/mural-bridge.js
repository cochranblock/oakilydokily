// Bridge: pass scroll and mouse from JS into mural-wasm
(function() {
  if (typeof miniquad_add_plugin !== 'function') {
    console.error('mural-bridge: miniquad_add_plugin not found. Load gl.js first.');
    return;
  }
  miniquad_add_plugin({
    on_init: function() {
      if (typeof window.mural_ready === 'function') window.mural_ready();

      // Fetch pixel-forge sprites from kova node and load into WASM
      (function fetchForgedSprites() {
        var CELL_W = 16, CELL_H = 16;
        fetch('/api/forge', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ class: 'animal', palette: 'stardew', count: 8, steps: 40 })
        })
        .then(function(r) { return r.json(); })
        .then(function(json) {
          if (!json.ok || !json.data || !json.data.sprites) {
            console.warn('forge: bad response', json);
            return;
          }
          var sprites = json.data.sprites;
          var count = sprites.length;
          if (count === 0) return;

          // Decode base64 PNGs to RGBA via offscreen canvas
          var sheetW = count * CELL_W;
          var sheetH = CELL_H;
          var canvas = document.createElement('canvas');
          canvas.width = sheetW;
          canvas.height = sheetH;
          var ctx = canvas.getContext('2d');

          var loaded = 0;
          sprites.forEach(function(sprite, i) {
            var img = new Image();
            img.onload = function() {
              ctx.drawImage(img, i * CELL_W, 0, CELL_W, CELL_H);
              loaded++;
              if (loaded === count) {
                // All sprites decoded — push RGBA to WASM
                var rgba = ctx.getImageData(0, 0, sheetW, sheetH).data;
                var len = rgba.length;
                var ptr = wasm_exports.mural_alloc(len);
                var mem = new Uint8Array(wasm_memory.buffer, ptr, len);
                mem.set(rgba);
                wasm_exports.mural_load_sprites(ptr, len, count, CELL_W, CELL_H);
                console.log('forge: loaded', count, 'sprites into WASM');
              }
            };
            img.onerror = function() {
              console.warn('forge: failed to decode sprite', i);
              loaded++;
            };
            img.src = 'data:image/png;base64,' + sprite.png_b64;
          });
        })
        .catch(function(e) { console.warn('forge: fetch failed', e); });
      })();

      setInterval(function() {
        try {
          if (wasm_exports && wasm_exports.mural_set_scroll_x) wasm_exports.mural_set_scroll_x(window.scrollX);
          if (wasm_exports && wasm_exports.mural_set_scroll_y) wasm_exports.mural_set_scroll_y(window.scrollY);
        } catch (e) { console.warn('mural-bridge scroll:', e); }
      }, 16);
      var c = document.getElementById('glcanvas');
      if (c && wasm_exports && typeof wasm_exports.mural_set_mouse === 'function') {
        c.addEventListener('mousemove', function(e) {
          try {
            var r = c.getBoundingClientRect();
            var w = r.width || 1, h = r.height || 1;
            var cw = c.width || 1, ch = c.height || 1;
            wasm_exports.mural_set_mouse((e.clientX - r.left) * (cw / w), (e.clientY - r.top) * (ch / h));
          } catch (err) { console.warn('mural-bridge mouse:', err); }
        });
      }
    }
  });
})();
