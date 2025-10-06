#include <Arduino.h>

// Row and column pins
const int rows[5] = {4, 5, 16, 17, 18};
const int cols[5] = {12, 13, 14, 15, 21};

// Debounce settings
const unsigned long debounceDelay = 50; // milliseconds

// Store last read state and last debounce time for each button
bool buttonState[5][5];
bool lastButtonState[5][5];
unsigned long lastDebounceTime[5][5];

void setup() {
  Serial.begin(115200);

  // Initialize rows
  for (int i = 0; i < 5; i++) {
    pinMode(rows[i], OUTPUT);
    digitalWrite(rows[i], HIGH);
  }

  // Initialize columns
  for (int i = 0; i < 5; i++) {
    pinMode(cols[i], INPUT_PULLUP);
  }

  // Initialize button states
  for (int r = 0; r < 5; r++) {
    for (int c = 0; c < 5; c++) {
      buttonState[r][c] = HIGH; // not pressed
      lastButtonState[r][c] = HIGH;
      lastDebounceTime[r][c] = 0;
    }
  }
}

void loop() {
  unsigned long currentTime = millis();

  for (int r = 0; r < 5; r++) {
    // Activate current row
    digitalWrite(rows[r], LOW);

    for (int c = 0; c < 5; c++) {
      int reading = digitalRead(cols[c]);

      if (reading != lastButtonState[r][c]) {
        // Reset debounce timer
        lastDebounceTime[r][c] = currentTime;
      }

      if ((currentTime - lastDebounceTime[r][c]) > debounceDelay) {
        // If the reading has changed for longer than debounce delay
        if (reading != buttonState[r][c]) {
          buttonState[r][c] = reading;

          // Only trigger on button press (LOW)
          if (buttonState[r][c] == LOW) {
            Serial.print("Button ");
            Serial.print(r + 1);
            Serial.print("|");
            Serial.println(c + 1);
          }
        }
      }

      lastButtonState[r][c] = reading;
    }

    // Deactivate row
    digitalWrite(rows[r], HIGH);
  }

  delay(10); // small delay for stability
}
