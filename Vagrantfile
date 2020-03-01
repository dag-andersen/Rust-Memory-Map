# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
    config.vm.box = 'digital_ocean'
    config.vm.box_url = "https://github.com/devopsgroup-io/vagrant-digitalocean/raw/master/box/digital_ocean.box"
    config.ssh.private_key_path = '~/.ssh/id_rsa'
    config.vm.synced_folder ".", "/vagrant", type: "rsync",
        rsync__exclude: ["/target/", ".git/", ".github/", ".docs/", ".vscode", "ITU-STUFF/"]

    config.vm.define "rust-memory-map", primary: true do |server|
      server.vm.provider :digital_ocean do |provider|
        provider.ssh_key_name = ENV["SSH_KEY_PRIVATE_NAME"]
        provider.token = ENV["DIGITAL_OCEAN_PRIVATE_TOKEN"]
        provider.image = 'ubuntu-18-04-x64'
        provider.region = 'fra1'
        provider.size = '8gb'
        provider.privatenetworking = true
      end

      server.vm.hostname = "rust-memory-map"

      server.vm.provision "shell", inline: <<-SHELL
        echo "----- Running: cd ../vagrant/"
        cd ../vagrant/
        echo "----- Current Directory is now: "$PWD
        echo "----- apt update"
        sudo apt update
        echo "----- sudo apt --yes install rustc"
        sudo apt --yes install rustc
        echo "----- sudo apt --yes install cargo"
        sudo apt --yes install cargo
      SHELL
    end

    config.vm.provision "shell", privileged: false, inline: <<-SHELL
      sudo apt-get update
    SHELL
  end