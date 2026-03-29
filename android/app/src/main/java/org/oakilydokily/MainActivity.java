// Unlicense — cochranblock.org
// OakilyDokily Pocket Server: WebView wrapper for embedded Rust web server

package org.oakilydokily;

import android.app.Activity;
import android.content.Intent;
import android.os.Bundle;
import android.webkit.WebView;
import android.webkit.WebViewClient;
import android.webkit.WebSettings;

public class MainActivity extends Activity {
    private WebView webView;
    private static final String SERVER_URL = "http://127.0.0.1:3000";

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        // Start the Rust server as a foreground service
        Intent svc = new Intent(this, ServerService.class);
        startForegroundService(svc);

        // Wait for server to bind (native lib init)
        try { Thread.sleep(1500); } catch (InterruptedException ignored) {}

        // WebView pointed at localhost
        webView = new WebView(this);
        WebSettings s = webView.getSettings();
        s.setJavaScriptEnabled(false); // Zero JS — server-rendered HTML
        s.setDomStorageEnabled(false);
        s.setCacheMode(WebSettings.LOAD_NO_CACHE);
        webView.setWebViewClient(new WebViewClient());
        webView.loadUrl(SERVER_URL);
        setContentView(webView);
    }

    @Override
    public void onBackPressed() {
        if (webView != null && webView.canGoBack()) {
            webView.goBack();
        } else {
            super.onBackPressed();
        }
    }

    @Override
    protected void onDestroy() {
        super.onDestroy();
        stopService(new Intent(this, ServerService.class));
    }
}
