package com.example.smsgateway;

import android.content.Context;

public class NativeBridge {
    public static native void init(Context context);
    public static native void startServer(int port);
    public static native void stopServer();
    public static native void onSmsReceived(String from, String body);
}
