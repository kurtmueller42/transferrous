cargo build
rm -r ./test_directory/
mkdir ./test_directory
cp ./testfiles/* ./test_directory/
mv -f ./target/debug/transferrous-server ./test_directory
mv -f ./target/debug/transferrous-client ./test_directory
mv -f ./target/debug/transferrous-client-gui .
