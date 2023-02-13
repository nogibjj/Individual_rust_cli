# rust-new-project-template
## Introduction
This is a course project from IDS721 cloud computing. This project aims to develop a useful microservice for data engineers based on Rust. In addition, this project will be deployed on AWS. Specifically, user can just type into two location names and get useful information like Lat and Lng, travel time, etc.

## Usage
For local testing, user must use their own GogleMap API Key to succesful access the information. Or user can use the link to use the mapservice.

For local testing, cd to the mapservice folder and type in:
```
cargo run
```

For docker, type in:
```
make build
make rundocker
```
## Project Screenshots
For distance:
![alt text](src/sh1.png)
For time:
![alt text](src/sh2.png)
For Lat and Lng:
![alt text](src/sh3.png)
## Project plan
### Week One:
Start Projects and finish the basic functions.
### Week Two:
Finish the basic functions and test it with docker.
### Week Three:
Fix bugs and depoly on AWS and test it.


## References
* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
