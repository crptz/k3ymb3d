RED='\033[0;31m'
WHITE='\033[1;37m'

printf "Executing cargo build...\n"

cargo build

printf "${RED}WARNING!${WHITE} YOU ARE GOING RUN AS ROOT...\n"

printf "Continue?(y/n)"

read answer

if [[ "$answer" = "y" ]]
then
	printf "OK!\n"
else
	printf "Exiting..."
	sleep 3
	exit 0
fi

sudo ./target/debug/k3ymb3d
