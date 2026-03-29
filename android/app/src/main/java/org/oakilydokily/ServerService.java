// Unlicense — cochranblock.org
// Foreground service that runs the Rust oakilydokily server

package org.oakilydokily;

import android.app.Notification;
import android.app.NotificationChannel;
import android.app.NotificationManager;
import android.app.Service;
import android.content.Intent;
import android.os.IBinder;

public class ServerService extends Service {
    private static final String CHANNEL_ID = "oakilydokily_server";
    private Thread serverThread;

    static {
        System.loadLibrary("oakilydokily");
    }

    // JNI entry point — implemented in Rust via jni crate
    private static native void startServer(String dataDir, int port);

    @Override
    public void onCreate() {
        super.onCreate();
        createNotificationChannel();

        Notification notification = new Notification.Builder(this, CHANNEL_ID)
            .setContentTitle("OakilyDokily")
            .setContentText("Server running on localhost:3000")
            .setSmallIcon(android.R.drawable.ic_menu_compass)
            .build();

        startForeground(1, notification);

        String dataDir = getFilesDir().getAbsolutePath();
        serverThread = new Thread(() -> startServer(dataDir, 3000));
        serverThread.setDaemon(true);
        serverThread.start();
    }

    @Override
    public IBinder onBind(Intent intent) { return null; }

    @Override
    public void onDestroy() {
        super.onDestroy();
        // Server thread is daemon — dies with service
    }

    private void createNotificationChannel() {
        NotificationChannel channel = new NotificationChannel(
            CHANNEL_ID, "Server", NotificationManager.IMPORTANCE_LOW);
        channel.setDescription("OakilyDokily server status");
        getSystemService(NotificationManager.class).createNotificationChannel(channel);
    }
}
