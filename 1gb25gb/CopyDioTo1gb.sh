scp -r daga@dionysos.itu.dk:/mnt/nvme/daga/cachefull/testdata /Volumes/1TB\ Hjerne/bachelorfolder

scp -r daga@dionysos.itu.dk:/mnt/nvme/daga/cachefull/input_data_shuffled.txt /Volumes/1TB\ Hjerne/bachelorfolder
scp -r /Volumes/1TB\ Hjerne/bachelorfolder/input_data_shuffled.txt root@178.128.154.165:/mnt/volume_nyc1_01

scp -r /Volumes/1TB\ Hjerne/bachelorfolder/testdata/out/tree root@178.128.154.165:/mnt/volume_nyc1_01/testdata/out/
scp -r /Volumes/1TB\ Hjerne/bachelorfolder/testdata/out/redblack root@178.128.154.165:/mnt/volume_nyc1_01/testdata/out/
scp -r /Volumes/1TB\ Hjerne/bachelorfolder/testdata/out/table root@178.128.154.165:/mnt/volume_nyc1_01/testdata/out/