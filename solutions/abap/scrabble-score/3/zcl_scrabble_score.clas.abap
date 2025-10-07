CLASS zcl_scrabble_score DEFINITION PUBLIC .

  PUBLIC SECTION.
    METHODS score
      IMPORTING
        input         TYPE string OPTIONAL
      RETURNING
        VALUE(result) TYPE i.
  PROTECTED SECTION.
  PRIVATE SECTION.

ENDCLASS.


CLASS zcl_scrabble_score IMPLEMENTATION.
  METHOD score.
    TYPES: BEGIN OF ty_characters_value_row,
      letter TYPE string,
      value TYPE i,
    END OF ty_characters_value_row,
    ty_characters_value_table TYPE TABLE OF ty_characters_value_row WITH EMPTY KEY.
    
    DATA(characters_value_table) = VALUE ty_characters_value_table(
                                            ( letter = 'A' value = 1 )
                                            ( letter = 'E' value = 1 )
                                            ( letter = 'I' value = 1 )
                                            ( letter = 'O' value = 1 )
                                            ( letter = 'U' value = 1 )
                                            ( letter = 'L' value = 1 )
                                            ( letter = 'N' value = 1 )
                                            ( letter = 'R' value = 1 )
                                            ( letter = 'S' value = 1 )
                                            ( letter = 'T' value = 1 )
                                            ( letter = 'D' value = 2 )
                                            ( letter = 'G' value = 2 )
                                            ( letter = 'B' value = 3 )
                                            ( letter = 'C' value = 3 )
                                            ( letter = 'M' value = 3 )
                                            ( letter = 'P' value = 3 )
                                            ( letter = 'F' value = 4 )
                                            ( letter = 'H' value = 4 )
                                            ( letter = 'V' value = 4 )
                                            ( letter = 'W' value = 4 )
                                            ( letter = 'Y' value = 4 )
                                            ( letter = 'K' value = 5 )
                                            ( letter = 'J' value = 8 )
                                            ( letter = 'X' value = 8 )
                                            ( letter = 'Q' value = 10 )
                                            ( letter = 'Z' value = 10 )
                                         ).
    
    DO strlen( input ) TIMES.
      DATA(lv_index) = sy-index - 1. " Adjusting for zero-based index
      DATA(letter) = input+lv_index(1).
      DATA(letter_uppercase) = to_upper( letter ).
      DATA(row) = characters_value_table[ letter = letter_uppercase ].
      
      IF sy-subrc IS INITIAL.
        result = result + row-value.
      ENDIF.
    ENDDO.
  ENDMETHOD.

ENDCLASS.
