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
    DATA: lv_string_1 TYPE string VALUE 'AEIOULNRST',
          lv_string_2 TYPE string VALUE 'DG',
          lv_string_3 TYPE string VALUE 'BCMP',
          lv_string_4 TYPE string VALUE 'FHVWY',
          lv_string_5 TYPE string VALUE 'K',
          lv_string_6 TYPE string VALUE 'JX',
          lv_string_7 TYPE string VALUE 'QZ',
          lv_length   TYPE i,
          lv_index    TYPE i,
          lv_char     TYPE c LENGTH 1.

    result = 0.
    lv_length = strlen( input ).

    DO lv_length TIMES.
      lv_index = sy-index - 1. " Adjusting for zero-based index
      lv_char = to_upper( input+lv_index(1) ) .

      IF lv_string_1 CA lv_char.
        result = result + 1.

        ELSEIF lv_string_2 CA lv_char.
          result = result + 2.

        ELSEIF lv_string_3 CA lv_char.
          result = result + 3.

        ELSEIF lv_string_4 CA lv_char.
          result = result + 4.

        ELSEIF lv_string_5 CA lv_char.
          result = result + 5.

        ELSEIF lv_string_6 CA lv_char.
          result = result + 8.

        ELSEIF lv_string_7 CA lv_char.
          result = result + 10.
      ENDIF.
    ENDDO.
  ENDMETHOD.

ENDCLASS.
