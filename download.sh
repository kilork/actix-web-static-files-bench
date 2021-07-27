set -e

curl -v -C - --retry 999 -O https://cdn.kernel.org/pub/linux/kernel/v5.x/linux-5.10.9.tar.xz
rm -rf linux-5.10.9
rm example-01-linux-basic/linux-5.10.9
tar -xf linux-5.10.9.tar.xz
ln -s -f ${PWD}/linux-5.10.9 example-01-linux-basic/linux-5.10.9