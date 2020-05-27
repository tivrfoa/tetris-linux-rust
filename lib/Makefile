#OBJS specifies which files to compile as part of the project
OBJS = View.c

#CC specifies which compiler we're using
CC = gcc

#LDFLAGS specifies what we're linking against
LDFLAGS = -lSDL2 -lSDL2_gfx

#OBJ_NAME specifies the name of our exectuable
OBJ_NAME = Tetris

#This is the target that compiles our executable
all : $(OBJS)
	$(CC) -c $(OBJS) -o $(OBJ_NAME) $(LDFLAGS)
