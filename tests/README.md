## This contains integration testcases

### Dev test

Follow these steps to run docker container based setup

  * Build the docker images using build.sh script
  * Build your application module/dynamic library on Ubuntu 18.04 OS
  * Copy the application library (.so file) into all the config*/app directories
  * Add application library path to all the config files in config* directories
  * Run "docker-compose up" from docker dir
 
 The above command will spin-up 3 blockchain nodes (docker container).
 
 Config files for the nodes are placed in config1, config2 and config3 folders.
 
 To stop the containers run "docker-compose down" from docker dir

#### Change the number of nodes

If you want to change the number of nodes to run, a new service is to be added in the docker-compose file per extra node
