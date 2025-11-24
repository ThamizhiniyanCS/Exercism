CLASS zcl_kindergarten_garden DEFINITION
  PUBLIC
  FINAL
  CREATE PUBLIC .

  PUBLIC SECTION.
    METHODS plants
      IMPORTING
        diagram        TYPE string
        student        TYPE string
      RETURNING
        VALUE(results) TYPE string_table.

  PROTECTED SECTION.
  PRIVATE SECTION.
    DATA students TYPE string_table.

ENDCLASS.


CLASS zcl_kindergarten_garden IMPLEMENTATION.


  METHOD plants.
    DATA: lv_index TYPE i,
          student_index TYPE i,
          garden_itab TYPE TABLE OF string WITH EMPTY KEY,
          student_itab TYPE TABLE OF string WITH EMPTY KEY,
          row TYPE string,
          seeds TYPE string.

    student_itab = VALUE #(
      ( `Alice` )
      ( `Bob` )
      ( `Charlie` )
      ( `David` )
      ( `Eve` )
      ( `Fred` )
      ( `Ginny` )
      ( `Harriet` )
      ( `Ileana` )
      ( `Joseph` )
      ( `Kincaid` )
      ( `Larry` )      
    ).

    SPLIT diagram AT '\n' INTO TABLE garden_itab.

    student_index = line_index( student_itab[ table_line = student ] ).

    DO lines( garden_itab ) TIMES.
      row = garden_itab[ sy-index ].
      lv_index = ( student_index - 1 ) * 2.

      seeds = seeds && row+lv_index(2).
    ENDDO.

    DO strlen( seeds ) TIMES.
      lv_index = sy-index - 1.

      CASE seeds+lv_index(1).
        WHEN 'G'.
          APPEND 'grass' TO results.
        WHEN 'C'.
          APPEND 'clover' TO results.
        WHEN 'R'.
          APPEND 'radishes' TO results.
        WHEN 'V'.
          APPEND 'violets' TO results.
      ENDCASE.
    ENDDO.
  ENDMETHOD. 
ENDCLASS.