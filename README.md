# ray tracer

This is my try to create ray tracer on CPU

to build:
```
mkdir build
cd build
cmake .. -G  "<your engine generator>"
cmake --build .
```

This code should produce file named ray_tracer
To run this executable
```
./ray_tracer --source=<path to .obj file> --output=<name of output image.bmp>
```

**TODO**  
add another image format  
make camera view modifiable
