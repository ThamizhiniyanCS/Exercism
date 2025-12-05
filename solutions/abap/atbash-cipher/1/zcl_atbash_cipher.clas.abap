CLASS zcl_atbash_cipher DEFINITION.

  PUBLIC SECTION.
    METHODS decode
      IMPORTING
        cipher_text       TYPE string
      RETURNING
        VALUE(plain_text) TYPE string .
    METHODS encode
      IMPORTING
        plain_text         TYPE string
      RETURNING
        VALUE(cipher_text) TYPE string .
  PROTECTED SECTION.
  PRIVATE SECTION.
    CLASS-DATA: static_numbers_string TYPE string VALUE '0123456789',
                static_alphabets_string TYPE string VALUE 'abcdefghijklmnopqrstuvwxyz'.
ENDCLASS.

CLASS zcl_atbash_cipher IMPLEMENTATION.

  METHOD decode.
    DATA: lv_index          TYPE i,
          lv_current_value  TYPE c,
          lv_alphabet_index TYPE i.

    DO strlen( cipher_text ) TIMES.
      lv_index = sy-index - 1.
      lv_current_value = cipher_text+lv_index(1).

      IF lv_current_value CO static_numbers_string.
        plain_text = plain_text && lv_current_value.
      ELSEIF lv_current_value CO static_alphabets_string.
        FIND lv_current_value IN static_alphabets_string MATCH OFFSET lv_alphabet_index.

        IF sy-subrc = 0.
          DATA(replacing_index) = 26 - lv_alphabet_index - 1.
          plain_text = plain_text && static_alphabets_string+replacing_index(1).
        ENDIF.
      ENDIF.
    ENDDO.
  ENDMETHOD.

  METHOD encode.
    DATA: lv_index          TYPE i,
          lv_counter        TYPE i,
          lv_current_value  TYPE c,
          lv_alphabet_index TYPE i.

    DATA(lv_plain_text) = to_lower( plain_text ).
    DATA(lv_plain_text_len) = strlen( lv_plain_text ).

    DO lv_plain_text_len TIMES.
      IF lv_counter = 5 AND sy-index <> lv_plain_text_len.
        cipher_text = cipher_text && ` `.
        lv_counter = 0.
      ENDIF.

      lv_index = sy-index - 1.
      lv_current_value = lv_plain_text+lv_index(1).

      IF lv_current_value CO static_numbers_string.
        cipher_text = cipher_text && lv_current_value.
        lv_counter = lv_counter + 1.
      ELSEIF lv_current_value CO static_alphabets_string.
        FIND lv_current_value IN static_alphabets_string MATCH OFFSET lv_alphabet_index.

        IF sy-subrc = 0.
          DATA(replacing_index) = 26 - lv_alphabet_index - 1.
          cipher_text = cipher_text && static_alphabets_string+replacing_index(1).
          lv_counter = lv_counter + 1.
        ENDIF.
      ENDIF.
    ENDDO.
  ENDMETHOD.
ENDCLASS.