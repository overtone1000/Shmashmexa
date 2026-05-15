RPI5_BOARD_WIDTH=85;
RPI5_BOARD_HEIGHT=56;
RPI5_BOARD_DEPTH=21.2; //from forums

RPI5_MOUNTING_HOLE_ORIGIN=[3.5,3.5];
RPI5_MOUNTING_HOLE_X_DELTA=58;
RPI5_MOUNTING_HOLE_Y_DELTA=49;
RPI5_MOUNTING_HOLE_DIAMETER=2.7;
RPI5_MOUNTING_HOLES=[
    RPI5_MOUNTING_HOLE_ORIGIN,
    RPI5_MOUNTING_HOLE_ORIGIN+[RPI5_MOUNTING_HOLE_X_DELTA,0],
    RPI5_MOUNTING_HOLE_ORIGIN+[0,RPI5_MOUNTING_HOLE_Y_DELTA],
    RPI5_MOUNTING_HOLE_ORIGIN+[RPI5_MOUNTING_HOLE_X_DELTA,RPI5_MOUNTING_HOLE_Y_DELTA]
];

module rpi_mounting_hole_cuts(length=999,center=false,diameter=RPI5_MOUNTING_HOLE_DIAMETER,margin=0.4)
{
    module cuts()
    {
        for(i=[0:1])
        {
            for(j=[0:1])
            {
                xy=[
                    RPI5_MOUNTING_HOLE_ORIGIN[0]+RPI5_MOUNTING_HOLE_X_DELTA*i,
                    RPI5_MOUNTING_HOLE_ORIGIN[1]+RPI5_MOUNTING_HOLE_Y_DELTA*j
                ];
                translate ([xy[0],xy[1],0]) cylinder(h=length,d=diameter+margin,center=true);
            }
        }
    }
    
    if (center)
    {
        translate([-RPI5_BOARD_WIDTH/2,-RPI5_BOARD_HEIGHT/2,0]) cuts();
    }
    else
    {
        cuts();
    }
}

module rpi_board_model(center=false)
{
    module board() {
        difference()
        {
            cube([RPI5_BOARD_WIDTH,RPI5_BOARD_HEIGHT,RPI5_BOARD_DEPTH]);
            rpi_mounting_hole_cuts(999,false);
        }
    }
    
    if (center)
    {
        translate([-RPI5_BOARD_WIDTH/2,-RPI5_BOARD_HEIGHT/2,0]) board();
    }
    else
    {
        board();
    }
}

//rpi_board_model(false);
//rpi_mounting_hole_cuts(999,false);
