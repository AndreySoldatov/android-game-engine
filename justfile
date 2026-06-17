# Run the demo showcasing the custom virtual console widgets collection
widgets-demo-desktop:
    cargo run -p android-demo --release --bin desktop

android-demo-dir := "vcon-widgets/android-demo"
widgets-demo-android:
    @adb devices | awk 'NR > 1 && $2 == "device" { found = 1 } END { if (!found) { print "No connected authorized adb device found" > "/dev/stderr"; exit 1 } }'
    cd {{ android-demo-dir }} && cargo ndk -t arm64-v8a -o app/src/main/jniLibs/ build
    cd {{ android-demo-dir }} && ./gradlew build
    cd {{ android-demo-dir }} && ./gradlew installDebug
    adb shell am start -n com.andrey.vconwidgetsdemo/.MainActivity
