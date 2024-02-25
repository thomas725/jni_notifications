
ln -s target/release/libjni_notifications.so libjni_notifications.so
java -cp . -Djava.library.path=$(pwd) com.zoffcc.applications.jninotifications.NTFYActivity
rm libjni_notifications.so