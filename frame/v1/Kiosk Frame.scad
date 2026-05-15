$fn = $preview ? 16 : 128;

include <../@Commons/Constants.scad>;
include <../@Commons/RPi5Specs.scad>;

ARBITRARILY_LARGE=999;
PERTURB=0.01;

mm_per_inch=25.4;

MINK_RADIUS=0.75;

//For HAMTYSAN 10 inch 1024x600 mini monitor
SCREEN_WIDTH=9.25*mm_per_inch; //from docs
SCREEN_HEIGHT=5.62*mm_per_inch; //from docs
SCREEN_DEPTH=0.27*mm_per_inch; //from docs
SCREEN_RECESS_ALLOWANCE=0.2;
THREADED_THICKNESS=8.95-SCREEN_DEPTH; //measured
SCREW_THREAD_X=[23.2,81.2]; //measured
SCREW_THREAD_Y=[12.5,61.5]; //measured
SCREW_THREAD_INNER_DIAMETER=3; //measured, probably M3 bolt. Nope, not M3. M2.5?
SCREW_INSERT_OUTER_DIAMETER=9; //measured
MOUNT_THREAD_HOLE_DIAMETER=RPI5_MOUNTING_HOLE_DIAMETER+0.4;

CIRCUIT_BOARD_X=0;
CIRCUIT_BOARD_Y=9;
CIRCUIT_BOARD_WIDTH=140;
CIRCUIT_BOARD_HEIGHT=56;
CIRCUIT_BOARD_DEPTH=4.5;

HDMI_PORT_DEPTH=11.5;
HDMI_PORT_WIDTH=15;
HDMI_PORT_HEIGHT=5.5;
HDMI_PORT_RIGHT_X=3.2;
HDMI_PORT_BOTTOM_Y=40;
HDMI_PORT_TOP_Z=18.8-SCREEN_DEPTH;

USB_PORT_DEPTH=6.5;
USB_PORT_WIDTH=7.5;
USB_PORT_HEIGHT=3;
USB_PORT_RIGHT_X=1;
USB_PORT_BOTTOM_Y=25;
USB_PORT_TOP_Z=15.6-SCREEN_DEPTH;

BASE_DEPTH=2;
BASE_HEIGHT=90;
BASE_ANGLE=-75; //-75 looks good

FRAME_OUTER_THICKNESS=16; //minimum with adapter for HDMI measures 13

RPI_X_OFFSET=(SCREEN_WIDTH+FRAME_OUTER_THICKNESS*2)/2;
RPI_Y_OFFSET=BASE_HEIGHT/2+8;
RPI_Z_OFFSET=BASE_DEPTH;

DOOR_WIDTH=SCREEN_WIDTH/2;
DOOR_LENGTH=SCREEN_HEIGHT;
DOOR_DEPTH=2.2;
DOOR_VERTICAL_MARGIN=5;
DOOR_X=RPI_X_OFFSET;
DOOR_Y=FRAME_OUTER_THICKNESS-BASE_DEPTH;

//Back dimension and angle calculation
BACK_WIDTH=SCREEN_WIDTH+FRAME_OUTER_THICKNESS*2;
FUDGE_FACTOR=2;
MIDDLE_LINE_LENGTH=sin(BASE_ANGLE)*(BASE_HEIGHT+FUDGE_FACTOR);
NEXT_TO_BASE_LINE_LENGTH=(BASE_HEIGHT^2-MIDDLE_LINE_LENGTH^2)^0.5;
NEXT_TO_TOP_LINE_LENGTH=SCREEN_HEIGHT+FRAME_OUTER_THICKNESS*2-NEXT_TO_BASE_LINE_LENGTH;
BACK_LENGTH=(NEXT_TO_TOP_LINE_LENGTH^2+MIDDLE_LINE_LENGTH^2)^0.5;
BACK_ANGLE=-asin(MIDDLE_LINE_LENGTH/BACK_LENGTH);
BACK_DEPTH=2;
SLAT_OVERLAP=5;
DOOR_OVERLAP=7;
SLAT_THICKNESS=DOOR_DEPTH*2;
SLAT_HEIGHT=DOOR_LENGTH-SLAT_OVERLAP;

USB_PORT_DIAMETER=24.5; //mm
USB_PORT_TRANSLATION=[USB_PORT_DIAMETER/2+20,BACK_LENGTH-40-USB_PORT_DIAMETER/2,0];
MAX_DEPTH=6; //mm (for USB port to fit)
    
assert(BACK_DEPTH<=MAX_DEPTH,"Back depth is too high, USB port will not fit without adding additional recess.");


module mink_cube(size=[width,height,depth],center=false)
{
    module shape()
    {
        minkowski(convexity=5)
        {
            cube([size[0]-MINK_RADIUS*2,size[1]-MINK_RADIUS*2,size[2]-MINK_RADIUS*2],center);
            sphere(r=MINK_RADIUS);
        }
    }
    
    if(center)
    {
        shape();
    }
    else
    {
        translate([MINK_RADIUS,MINK_RADIUS,MINK_RADIUS])
        shape();
    }
};

module mount_screws(only_render_bolt_cut=false)
{
    for(i=[0:1])
    {
        for(j=[0:1])
        {
            translate(
                [
                    -SCREW_THREAD_X[i]+SCREEN_WIDTH/2,
                    SCREW_THREAD_Y[j]-SCREEN_HEIGHT/2,
                    -THREADED_THICKNESS/2
                ])
                difference(){
                    if(!only_render_bolt_cut) cylinder(h=THREADED_THICKNESS,d=SCREW_INSERT_OUTER_DIAMETER,center=true);
                    cylinder(h=ARBITRARILY_LARGE,d=MOUNT_THREAD_HOLE_DIAMETER,center=true);
                }
        }
    }
}

module screen_model()
{
    module screen()
    {
        translate ([0,0,SCREEN_DEPTH/2])
        cube(
            [
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                SCREEN_DEPTH
            ],
            center=true
        );
    }
    
    module ports()
    {
    
        module port(depth, width, height, right_x, bottom_y, top_z)
        {
            translate ([
                -SCREEN_WIDTH/2+depth/2-right_x,
                width/2-SCREEN_HEIGHT/2+bottom_y,
                height/2-top_z
            ])
            cube(
                [
                    depth,
                    width,
                    height
                ],
                center=true
            );
        }
        union()
        {
            port(HDMI_PORT_DEPTH, HDMI_PORT_WIDTH, HDMI_PORT_HEIGHT, HDMI_PORT_RIGHT_X, HDMI_PORT_BOTTOM_Y, HDMI_PORT_TOP_Z);
            port(USB_PORT_DEPTH, USB_PORT_WIDTH, USB_PORT_HEIGHT, USB_PORT_RIGHT_X, USB_PORT_BOTTOM_Y, USB_PORT_TOP_Z);
        }
    }
    
    module circuit_board()
    {
        translate ([
            CIRCUIT_BOARD_X-SCREEN_WIDTH/2,
            CIRCUIT_BOARD_Y-SCREEN_HEIGHT/2,
            -CIRCUIT_BOARD_DEPTH
        ])
        cube(
            [
                CIRCUIT_BOARD_WIDTH,
                CIRCUIT_BOARD_HEIGHT,
                CIRCUIT_BOARD_DEPTH
            ],
            center=false
        );
    }
    
    union()
    {
        color("silver", 0.5) screen();
        color("green",0.5) ports();
        color("green", 0.5) circuit_board();
        color("orange", 0.5) mount_screws();
    }
}

BOLT_LENGTH=4;
FASTENER_THICKNESS=BOLT_LENGTH-THREADED_THICKNESS;

module sub_hexes(hex_radius=2.5, wall_thickness=1.5, iterations=160)
{    
    step_angle=360/6;
    inner_radius=cos(step_angle/2)*hex_radius;
    
    HEXAGON = [
        for(i=[0:5])
        [sin(step_angle*i)*hex_radius,cos(step_angle*i)*hex_radius]
    ];
    
    I_STEP=inner_radius*2+wall_thickness;
    J_STEP=inner_radius*2;
    
    static_translation=[-iterations*I_STEP/2,-iterations*I_STEP/2];
    for(i=[0:iterations])
    {
        for(j=[0:iterations])
        {
            final_translation=static_translation+[i*I_STEP+j%2*I_STEP/2,j*J_STEP];
            translate(final_translation)
            polygon(HEXAGON);
        }
    }
}

module base_transform()
{
    translate([
        0,
        -SCREEN_HEIGHT/2-FRAME_OUTER_THICKNESS-sin(BASE_ANGLE)*BASE_DEPTH,
        SCREEN_DEPTH
    ])
    rotate([
        BASE_ANGLE,
        0,
        0
    ])
    translate([
        -SCREEN_WIDTH/2-FRAME_OUTER_THICKNESS,
        0,
        -BASE_DEPTH
    ])
    children();
}

module rpi_transform()
{
    base_transform()
    translate([RPI_X_OFFSET+RPI5_MOUNTING_HOLE_ORIGIN[0]-RPI5_MOUNTING_HOLE_X_DELTA/2,RPI_Y_OFFSET,RPI_Z_OFFSET])
    children();
}

module rpi5model()
{
    color("cyan",0.5) 
    rpi_transform()
    rpi_board_model(true);
    //rpi_mounting_hole_cuts(999,false);
}   

module frame()
{        
    module perimeter()
    {
        FRAME_MARGIN=0.5;
        BACKING_THICKNESS=3;
        BACKING_MARGIN=6;
        union()
        {
            translate([0,0,(SCREEN_DEPTH+SCREEN_RECESS_ALLOWANCE)/2])
            difference()
            {
                mink_cube([
                        SCREEN_WIDTH+FRAME_OUTER_THICKNESS*2,
                        SCREEN_HEIGHT+FRAME_OUTER_THICKNESS*2,
                        SCREEN_DEPTH+SCREEN_RECESS_ALLOWANCE
                    ],center=true);
                translate([0,0,0])
                cube([
                    SCREEN_WIDTH+FRAME_MARGIN,
                    SCREEN_HEIGHT+FRAME_MARGIN,
                    ARBITRARILY_LARGE
                ],center=true);
                
                ITERATIONS_X=20;
                ITERATIONS_Y=15;
                for(i=[0:ITERATIONS_X])
                {
                    for(j=[0:ITERATIONS_Y])
                    {
                        if(i==0||j==0||i==ITERATIONS_X||j==ITERATIONS_Y)
                        {
                            translate([
                                -SCREEN_WIDTH/2-FRAME_OUTER_THICKNESS/2,
                                -SCREEN_HEIGHT/2-FRAME_OUTER_THICKNESS/2])
                            translate([
                                (SCREEN_WIDTH+FRAME_OUTER_THICKNESS)*i/ITERATIONS_X,
                                (SCREEN_HEIGHT+FRAME_OUTER_THICKNESS)*j/ITERATIONS_Y])
                            cylinder(h=ARBITRARILY_LARGE,d=FRAME_OUTER_THICKNESS*3/8,center=true);
                        }
                    }
                }
            }
            difference()
            {
                translate([0,0,-BACKING_THICKNESS/2])
                cube([SCREEN_WIDTH+FRAME_OUTER_THICKNESS,SCREEN_HEIGHT+FRAME_OUTER_THICKNESS/2,BACKING_THICKNESS],center=true);
                cube([
                    SCREEN_WIDTH-BACKING_MARGIN*2,
                    SCREEN_HEIGHT-BACKING_MARGIN*2,
                    ARBITRARILY_LARGE
                ],center=true);
                //Screen circuit board
                CIRCUIT_BOARD_Y_MARGIN=2;
                translate ([
                    CIRCUIT_BOARD_X-SCREEN_WIDTH/2,
                    CIRCUIT_BOARD_Y-SCREEN_HEIGHT/2-CIRCUIT_BOARD_Y_MARGIN,
                    -CIRCUIT_BOARD_DEPTH
                ])
                cube(
                    [
                        CIRCUIT_BOARD_WIDTH,
                        CIRCUIT_BOARD_HEIGHT+CIRCUIT_BOARD_Y_MARGIN*2,
                        ARBITRARILY_LARGE
                    ],
                    center=false
                );                
            }
        }
    }
    
    module fastener()
    {
        MARGIN=6;
        WIDTH=SCREW_THREAD_X[1]-SCREW_THREAD_X[0]+MARGIN*2;
        HEIGHT=SCREEN_HEIGHT+FRAME_OUTER_THICKNESS;
        union()
        {
            translate([
                SCREEN_WIDTH/2-SCREW_THREAD_X[1]-MARGIN,
                -SCREEN_HEIGHT/2-FRAME_OUTER_THICKNESS,
                -FASTENER_THICKNESS-THREADED_THICKNESS
            ])
            cube([
                WIDTH,
                FRAME_OUTER_THICKNESS,
                FASTENER_THICKNESS+SCREEN_DEPTH
            ]);
            difference()
            {
                TRANSLATION=[
                    SCREEN_WIDTH/2-SCREW_THREAD_X[1]-MARGIN+WIDTH/2,
                    -SCREEN_HEIGHT/2-FRAME_OUTER_THICKNESS+HEIGHT/2,
                    -FASTENER_THICKNESS-THREADED_THICKNESS+FASTENER_THICKNESS/2
                ];
                translate(TRANSLATION)
                cube([
                    WIDTH,
                    HEIGHT,
                    FASTENER_THICKNESS
                ],center=true);
                mount_screws(true);
                translate(TRANSLATION)
                cube([
                    WIDTH*0.6,
                    HEIGHT*2,
                    ARBITRARILY_LARGE
                ],center=true);
                
            }
        }
    }
    
    module base()
    {
        B_WIDTH=SCREEN_WIDTH+FRAME_OUTER_THICKNESS*2;
        B_HEIGHT=BASE_HEIGHT+BACK_DEPTH;
        B_DEPTH=BASE_DEPTH;
        
        CUT_HEIGHT=B_HEIGHT*0.7;
        module cutter(START_X, END_X)
        {
            CUT_WIDTH=END_X-START_X;
            translate([
                START_X,
                (BASE_HEIGHT-CUT_HEIGHT)/2,
                0
            ])
            cube([
                CUT_WIDTH,
                CUT_HEIGHT,
                ARBITRARILY_LARGE
            ],center=false);
        }
        
        union()
        {
            LEFT_HOLES_X=RPI_X_OFFSET-RPI5_MOUNTING_HOLE_ORIGIN[0];
            RIGHT_HOLES_X=LEFT_HOLES_X-RPI5_MOUNTING_HOLE_X_DELTA;
            MOUNT_MARGIN=5;
            
            X1=FRAME_OUTER_THICKNESS;
            X2=RIGHT_HOLES_X-MOUNT_MARGIN*1.7;
            X3=RIGHT_HOLES_X+MOUNT_MARGIN*0.5;
            X4=LEFT_HOLES_X-MOUNT_MARGIN*1.7;
            X5=LEFT_HOLES_X+MOUNT_MARGIN*0.5;
            X6=X1+SCREEN_WIDTH;
            X_CUTS=[
                [X1,X2],
                [X3,X4],
                [X5,X6]
            ];
            
            base_transform()
            difference()
            {
                mink_cube(
                [
                    B_WIDTH,
                    B_HEIGHT,
                    B_DEPTH
                ],center=false);
                for(i=X_CUTS)
                {
                    cutter(i[0],i[1]);
                }
            }
         }
    }
    
    module base_cut()
    {
        base_transform()
        translate([-ARBITRARILY_LARGE/2,-ARBITRARILY_LARGE/2,-ARBITRARILY_LARGE+PERTURB])
        cube(ARBITRARILY_LARGE);
    }
    
        
    BASE_LENGTH=abs(cos(BASE_ANGLE)*BASE_HEIGHT);
    module sides()
    {        
        BACK_LENGTH=abs(sin(BASE_ANGLE)*BASE_HEIGHT);
        
        
        SIDE_THICKNESS=3;
        POINTS=[
            [BASE_DEPTH,BASE_DEPTH],
            [BASE_DEPTH,SCREEN_HEIGHT+FRAME_OUTER_THICKNESS*2-BASE_DEPTH],
            [BACK_LENGTH-BACK_DEPTH/2,BASE_LENGTH-BACK_DEPTH/2]
        ];
        
        for(xtranslate=[
            SCREEN_WIDTH/2+FRAME_OUTER_THICKNESS-SIDE_THICKNESS,
            -1*(SCREEN_WIDTH/2+FRAME_OUTER_THICKNESS)
        ])
        {
            translate([
                xtranslate,
                -SCREEN_HEIGHT/2-FRAME_OUTER_THICKNESS+BASE_DEPTH,
                SCREEN_DEPTH/2]
            )
            rotate([0,90,0])
            linear_extrude(SIDE_THICKNESS)
            {
                difference(){
                    polygon(POINTS);
                    translate([SCREEN_WIDTH/4,SCREEN_HEIGHT/1.5,0])
                    {
                        sub_hexes();
                    }
                }
            }
        }
    }
    
    
    module back()
    {        
        module door_opening_cutter()
        {
            translate([DOOR_X+DOOR_OVERLAP,DOOR_Y,-ARBITRARILY_LARGE/2])
            cube([DOOR_WIDTH-DOOR_OVERLAP*2,DOOR_LENGTH-DOOR_VERTICAL_MARGIN,ARBITRARILY_LARGE]);
        }
        
        SLAT_TRANSLATION=[
                        DOOR_X-SLAT_OVERLAP,
                        DOOR_Y+DOOR_LENGTH-SLAT_HEIGHT+SLAT_OVERLAP,
                        BACK_DEPTH-PERTURB];
        
        translate([0,SCREEN_HEIGHT/2+FRAME_OUTER_THICKNESS,0])
        rotate([BACK_ANGLE+180,0,0])
        translate([-BACK_WIDTH/2,0,-BACK_DEPTH])
        
        difference()
        {
            union()
            {
                difference()
                {
                    mink_cube([BACK_WIDTH,BACK_LENGTH,BACK_DEPTH]);

                    //Add door opening here
                    door_opening_cutter();
                    //USB Port
                    translate(USB_PORT_TRANSLATION)
                    cylinder(d=USB_PORT_DIAMETER, h=ARBITRARILY_LARGE,center=true);
                    //Hexes
                    linear_extrude(height=ARBITRARILY_LARGE,center=true)
                    {
                        translate([FRAME_OUTER_THICKNESS,FRAME_OUTER_THICKNESS,0])
                        intersection()
                        {
                            difference(){
                                square([SCREEN_WIDTH/2-SLAT_OVERLAP,SCREEN_HEIGHT-FRAME_OUTER_THICKNESS/2],center=false);
                                translate(USB_PORT_TRANSLATION-[FRAME_OUTER_THICKNESS,FRAME_OUTER_THICKNESS])
                                circle(r=USB_PORT_DIAMETER);
                            }
                            sub_hexes();
                        }
                    }
                }
                //Add slot for door
                difference()
                {
                    translate(SLAT_TRANSLATION)
                    mink_cube([DOOR_WIDTH+SLAT_OVERLAP*2,SLAT_HEIGHT,SLAT_THICKNESS]);
                    door_opening_cutter();
                }
            }
            //Cut away door
            translate(SLAT_TRANSLATION)
            translate([SLAT_OVERLAP,SLAT_HEIGHT-DOOR_LENGTH-SLAT_OVERLAP,0])
            cube([DOOR_WIDTH,DOOR_LENGTH,DOOR_DEPTH]);
        }
    }
    
    difference()
    {
        union()
        {
            perimeter();
            fastener();
            base();
            sides();
            color("pink",0.75) back();
        }
        base_cut();
        
        rpi_transform()
        translate([0,0,-ARBITRARILY_LARGE/2+1])
        rpi_mounting_hole_cuts(ARBITRARILY_LARGE,true);
        
        rpi_transform()
        translate([0,0,-ARBITRARILY_LARGE/2-BACK_DEPTH/2])
        rpi_mounting_hole_cuts(ARBITRARILY_LARGE,true,diameter=6);
    }
}

module door(test=false)
{
    module makedoor()
    {
        CUT_SCALE=0.9;
        SQUARE_DIM=[DOOR_WIDTH*CUT_SCALE,DOOR_LENGTH*CUT_SCALE];
        difference()
        {
            mink_cube([DOOR_WIDTH,DOOR_LENGTH,DOOR_DEPTH]);
            linear_extrude(height=ARBITRARILY_LARGE,center=true)
            {
                intersection()
                {
                    translate([DOOR_WIDTH/2,DOOR_LENGTH/2]) square(SQUARE_DIM, center=true);
                    translate([DOOR_WIDTH/2,DOOR_LENGTH/2]) sub_hexes();
                }
            }
        }
    }
    if(test==true)
    {
        translate([0,SCREEN_HEIGHT/2+FRAME_OUTER_THICKNESS,0])
        rotate([BACK_ANGLE-180,0,0])
        translate([-BACK_WIDTH/2,0,+1])
        translate([
        DOOR_X,
        DOOR_Y,
        BACK_DEPTH-DOOR_DEPTH])
        //translate([
        //        DOOR_X,
        //        DOOR_Y,
        //        BACK_DEPTH-PERTURB])
        makedoor();
    }
    else
    {
        makedoor();
    }
}

module final()
{
    union()
    {
        //rpi5model();
        //screen_model();
        frame();
        color("purple",0.5) door(true);
    }
}
final();

//sub_hexes();

//rotate_for_jlc3dp() final();
//rotate_for_jlc3dp() door(false);