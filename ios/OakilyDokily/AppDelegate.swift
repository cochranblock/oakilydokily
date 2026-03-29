// Unlicense — cochranblock.org
// OakilyDokily Pocket Server: iOS wrapper
// Thin Swift bridge — calls Rust server on background thread, WebView on main

import UIKit
import WebKit

@_silgen_name("od_start_server")
func od_start_server(_ dataDir: UnsafePointer<CChar>, _ port: Int32)

@main
class AppDelegate: UIResponder, UIApplicationDelegate {
    var window: UIWindow?

    func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?
    ) -> Bool {
        let dataDir = NSSearchPathForDirectoriesInDomains(.documentDirectory, .userDomainMask, true).first!

        // Start Rust server on background thread
        DispatchQueue.global(qos: .userInitiated).async {
            dataDir.withCString { ptr in
                od_start_server(ptr, 3000)
            }
        }

        // Wait for server to bind
        Thread.sleep(forTimeInterval: 1.5)

        // WebView on main thread
        let webView = WKWebView(frame: UIScreen.main.bounds)
        webView.load(URLRequest(url: URL(string: "http://127.0.0.1:3000")!))

        window = UIWindow(frame: UIScreen.main.bounds)
        let vc = UIViewController()
        vc.view = webView
        window?.rootViewController = vc
        window?.makeKeyAndVisible()

        return true
    }
}
