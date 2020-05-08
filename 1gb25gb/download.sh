set -e

vm="root@128.199.61.84:/mnt/vol1gbv2"

echo "Download - start"
scp -r $vm/speed/1gb25gb2v ./speed
scp -r $vm/cache/1gb25gb2v ./cache
echo "Download - complete"

