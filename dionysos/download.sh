set -e

vm="daga@dionysos.itu.dk:/mnt/nvme/daga"

echo "Download - start"
scp -r $vm/speed/dionysos ./speed
scp -r $vm/cache/dionysos ./cache
echo "Download - complete"
