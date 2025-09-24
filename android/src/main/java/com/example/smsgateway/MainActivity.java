package com.example.smsgateway;

import android.os.Bundle;
import android.widget.Button;
import android.widget.EditText;
import android.widget.TextView;
import androidx.appcompat.app.AppCompatActivity;

public class MainActivity extends AppCompatActivity {
    static {
        System.loadLibrary("smsgateway");
    }

    private boolean running = false;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        EditText portInput = findViewById(R.id.portInput);
        Button toggleButton = findViewById(R.id.toggleButton);
        TextView statusView = findViewById(R.id.statusView);

        toggleButton.setOnClickListener(v -> {
            if (running) {
                NativeBridge.stopServer();
                statusView.setText("Server stopped");
                toggleButton.setText("Start Server");
                running = false;
            } else {
                String port = portInput.getText().toString();
                NativeBridge.startServer(Integer.parseInt(port));
                statusView.setText("Server running on port " + port);
                toggleButton.setText("Stop Server");
                running = true;
            }
        });

        NativeBridge.init(this);
    }
}
