set -e

vm="root@167.71.1.68:/mnt/volume_ams3_01/"

echo "Download - start"
scp -r $vm/speed/2gb50gb ./speed
scp -r $vm/cache/2gb50gb ./cache
echo "Download - complete"

