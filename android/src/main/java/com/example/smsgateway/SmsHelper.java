package com.example.smsgateway;

import android.telephony.SmsManager;

public class SmsHelper {
    public static native void sendSms(String number, String message);

    public static void actuallySendSms(String number, String message) {
        SmsManager sms = SmsManager.getDefault();
        sms.sendTextMessage(number, null, message, null, null);
    }
}
