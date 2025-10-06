#include <Arduino.h>

// Row and column pins
const int rows[5] = {4, 5, 16, 17, 18};
const int cols[5] = {12, 13, 14, 15, 21};

void setup() {
  Serial.begin(115200);

  // Set row pins as OUTPUT and HIGH (inactive)
  for (int i = 0; i < 5; i++) {
    pinMode(rows[i], OUTPUT);
    digitalWrite(rows[i], HIGH);
  }

  // Set column pins as INPUT_PULLUP
  for (int i = 0; i < 5; i++) {
    pinMode(cols[i], INPUT_PULLUP);
  }
}

void loop() {
  for (int r = 0; r < 5; r++) {
    // Activate the current row
    digitalWrite(rows[r], LOW);

    // Scan columns
    for (int c = 0; c < 5; c++) {
      if (digitalRead(cols[c]) == LOW) { // Button pressed
        Serial.print("Button ");
        Serial.print(r + 1); // Row number starting at 1
        Serial.print("|");
        Serial.println(c + 1); // Col number starting at 1
      }
    }

    // Deactivate the row
    digitalWrite(rows[r], HIGH);
  }

  delay(100); // Small debounce / scan delay
}