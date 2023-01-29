package com.mjm.whisperkeyboardwatch;

import android.Manifest;
import android.content.Intent;
import android.os.Bundle;
import android.provider.Settings;
import android.widget.Button;
import android.widget.Toast;

import androidx.appcompat.app.AppCompatActivity;
import androidx.core.app.ActivityCompat;

public class Wizard extends AppCompatActivity {
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.wizard);

        Button btnEnableInputMethod = findViewById(R.id.btnEnableInputMethod);
        Button btnGrantMicrophonePermission = findViewById(R.id.btnGrantMicrophonePermission);
        Button btnExit = findViewById(R.id.btnExit);

        btnEnableInputMethod.setOnClickListener(v -> {
            Intent intent = new Intent(Settings.ACTION_INPUT_METHOD_SETTINGS);
            startActivity(intent);
        });

        btnGrantMicrophonePermission.setOnClickListener(v -> {
            ActivityCompat.requestPermissions(Wizard.this, new String[]{Manifest.permission.RECORD_AUDIO}, 1);
        });

        btnExit.setOnClickListener(v -> {
            Toast.makeText(this, R.string.wizard_completed, Toast.LENGTH_LONG).show();
            finish();
        });
    }
}