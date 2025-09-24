# webhook_to_sms
Run your own otp service from your mobile phone (pay to your ISP, through regular mobile recharge).



# SMS Gateway Android App

Turn your Android phone into a **local SMS gateway**. Send SMS messages via HTTP requests and receive incoming SMS in real time over WebSocket.

---

## 📦 Project Structure




smsgateway/

├── Cargo.toml

├── README.md

├── src/

│   ├── main.rs        # Dioxus UI

│   ├── server.rs      # HTTP + WebSocket server

│   ├── android.rs     # Rust ↔ Android bridge (JNI)

├── android/

│   └── src/main/java/com/example/smsgateway/

│       ├── MainActivity.java

│       ├── NativeBridge.java

│       ├── SmsHelper.java

│       ├── SmsReceiver.java

│   └── src/main/AndroidManifest.xml

│   └── res/layout/activity_main.xml





---

## ⚡ Features

1. **Send SMS** via the phone’s SIM card.  
2. **Receive SMS** in real time via WebSocket.  
3. **Editable WebSocket path** and HTTP port from the app UI.  
4. Simple mobile UI: toggle server, set port & WebSocket path, see status.  

---

## 🔐 Permissions

The app needs the following permissions:  

- `SEND_SMS` → to send messages  
- `RECEIVE_SMS` → to listen to incoming messages  
- `INTERNET` → to host HTTP/WebSocket server  

> Android will prompt you to allow these when the app starts.  

---

## 🛠 Build Instructions

### 1️⃣ Install Rust & Cargo NDK

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install cargo-ndk
cargo install cargo-ndk




2️⃣ Build Rust library for Android




# Navigate to the Rust project folder
cd smsgateway

# Build shared library for arm64 Android
cargo ndk -t arm64-v8a -o ./android/app/src/main/jniLibs build --release





This generates libsmsgateway.so for Android to call Rust code.





3️⃣ Open Android Studio




Open Android Studio → Open → select the android/ folder.




Make sure the libsmsgateway.so file is copied to android/app/src/main/jniLibs/arm64-v8a/.




Sync Gradle and build the project.




Run on your physical Android device (not emulator, since SMS sending needs real SIM).





📱 Using the App




Launch the app.




Enter a Port (e.g., 8080).




Enter WebSocket Path (e.g., /myws) — this is where incoming SMS will be streamed.




Press Start Server → status updates: Server running on 0.0.0.0:8080/myws.





🔹 Send SMS


From any device on the same network (phone, PC, server), send a POST request:




curl -X POST http://<phone_ip>:8080/v1/sms \
  -H "Content-Type: application/json" \
  -d '{"to":"+11234567890","message":"Hello!"}'





<phone_ip> → your Android phone IP on Wi-Fi.




to → recipient phone number (with country code).




message → the SMS content.





🔹 Receive SMS


Connect to the WebSocket to get incoming SMS in real time:




const ws = new WebSocket("ws://<phone_ip>:8080/myws");
ws.onmessage = (event) => console.log("Incoming SMS:", event.data);





The WebSocket URL uses the path set in the app.




Each message comes as JSON:






{
  "from": "+11234567890",
  "message": "Hello!"
}




⚠️ Notes




The app must stay open to keep the HTTP & WebSocket server running.




Make sure Wi-Fi is on and the phone IP is reachable.




The /v1/sms endpoint cannot be edited — this always sends SMS through your phone SIM.




Only the WebSocket path is user-editable.





✅ Summary




Send SMS: via /v1/sms POST → phone SIM.




Receive SMS: via WebSocket → real-time inbox events.




WebSocket path & port editable in UI.




Works offline on local network.


