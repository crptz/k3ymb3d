RED='\033[0;31m'
WHITE='\033[1;37m'
BLUE='\033[;34m'
END_COLOR='\033[0m'

printf "Executing cargo build...\n"

cargo build

printf "${RED}WARNING!${WHITE} YOU ARE GOING TO ${BLUE}SPACE...${END_COLOR}\n"

echo -e "Normal \e[5mBlink"

printf "Continue?(y/n)"

read answer

if (($answer = "y" || $answer = "Y" || $answer = ""))
then
	printf "OK!\n"
	printf "Running cargo run...\n"
	sleep 2
	cargo run
else
	printf "Exiting..."
	sleep 2
	exit 0
fi


