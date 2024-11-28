####################################### SOURCE ############################################
#rust image is compatible with x86 and arm
FROM rust:latest
LABEL authors="RITTEMARD.S FRADET.L"

####################################### ENVIRONNEMENT ############################################

# Set the working directory
WORKDIR /app

# Update package list and install software
RUN apt-get update
RUN apt-get upgrade -y
RUN apt-get install wget -y
RUN apt-get install unzip -y





# Copy files from host machine to the container
COPY ./src /app/src
COPY Cargo.toml /app
COPY ./assets /app/assets



#donwload the test images
RUN wget https://nasext-vaader.insa-rennes.fr/ietr-vaader/moseiik_test_images.zip -P tests/
RUN mkdir tests/moseiik_test_images
RUN unzip tests/moseiik_test_images.zip -d tests/moseiik_test_images/


####################################### PROCESS ############################################

ENTRYPOINT ["cargo"]

# By default, cargo run the program if we whant to test just add the arg test
CMD ["run", "--release", "--", "--image", "assets/target-small.png", "--tiles", "assets/tiles-small"]
