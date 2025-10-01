CLASS zcl_resistor_color DEFINITION PUBLIC CREATE PUBLIC.
  PUBLIC SECTION.
    METHODS resistor_color
      IMPORTING
        color_code   TYPE string
      RETURNING
        VALUE(value) TYPE i.
ENDCLASS.

CLASS zcl_resistor_color IMPLEMENTATION.

  METHOD resistor_color.
    TYPES ty_colors_table TYPE STANDARD TABLE OF string WITH EMPTY KEY.
    
    DATA(colors_table) = VALUE ty_colors_table(
                                                ( 'black' )
                                                ( 'brown' )
                                                ( 'red' )
                                                ( 'orange' )
                                                ( 'yellow' )
                                                ( 'green' )
                                                ( 'blue' )
                                                ( 'violet' )
                                                ( 'grey' )
                                                ( 'white' )
                                              ).

    value = line_index( colors_table[ table_line = color_code ] ) - 1.
  ENDMETHOD.

ENDCLASS.
