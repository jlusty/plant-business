#include <WiFi.h>
#include <WebServer.h>
#include <uri/UriRegex.h>
#include <HTTPClient.h>
#include <ArduinoJson.h>
#include "DHT.h"
#include "NetworkingConfig.h"

/* Sensor setup */
// Temperature, Humditiy
#define DHTPIN 4
#define DHTTYPE DHT22
DHT dht(DHTPIN, DHTTYPE);
float temperature = 0;
float humidity = 0;
// Light
int photoResPin = A4;
int lightValue = 0;
// Soil Moisture
int soilPin = A3;
float soilValue = 0;

/* Networking parameters */
// POST endpoint setup
unsigned long dataSentTime = 0;
unsigned long delayBetweenPOSTs = 10000; // 10 seconds

/* WebServer for dynamic configuration */
WebServer server(8032);

void setup()
{
  Serial.begin(115200);

  // Temp, Humidity
  dht.begin();
  // Light
  pinMode(photoResPin, INPUT);

  Serial.println("Connecting to ");
  Serial.println(wifiName);
  WiFi.begin(wifiName, wifiPassword);
  while (WiFi.status() != WL_CONNECTED)
  {
    delay(1000);
    Serial.print(".");
  }
  Serial.println("");
  Serial.print("WiFi connected, IP: ");
  Serial.println(WiFi.localIP());

  server.on(F("/"), []() {
    server.send(200, "text/plain", "ESP32 healthy!");
  });

  server.on(UriRegex("^\\/delay\\/([1-9][0-9]*)$"), HTTP_POST, []() {
    String delaySeconds = server.pathArg(0);
    delayBetweenPOSTs = delaySeconds.toInt() * 1000;
    server.send(200, "text/plain", "Set delay to " + String(delayBetweenPOSTs) + "ms");
  });

  server.begin();
  Serial.println("HTTP server started");
}

void loop()
{
  server.handleClient();
  delay(5);

  if ((millis() - dataSentTime) > delayBetweenPOSTs)
  {
    if (WiFi.status() == WL_CONNECTED)
    {
      Serial.println("Taking sensor data readings");
      temperature = dht.readTemperature();
      humidity = dht.readHumidity();
      lightValue = analogRead(photoResPin);
      soilValue = analogRead(soilPin);

      Serial.println("Sending sensor to server");
      HTTPClient http;
      http.begin(httpServerName);
      http.addHeader("Content-Type", "application/json");

      // Define JSON request body
      const int capacity = JSON_OBJECT_SIZE(4);
      StaticJsonDocument<capacity> doc;
      doc["temperature"] = temperature;
      doc["humidity"] = humidity;
      doc["light"] = lightValue;
      doc["soil_moisture"] = soilValue;
      char output[128];
      serializeJson(doc, output);

      int httpResponseCode = http.POST(output);

      Serial.print("HTTP response code is ");
      Serial.println(httpResponseCode);

      http.end();
    }
    else
    {
      Serial.println("WiFi Disconnected");
    }
    dataSentTime = millis();
  }
}
