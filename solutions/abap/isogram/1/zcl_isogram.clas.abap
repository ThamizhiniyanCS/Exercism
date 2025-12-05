CLASS zcl_isogram DEFINITION PUBLIC.

  PUBLIC SECTION.
    METHODS is_isogram
      IMPORTING
        VALUE(phrase)        TYPE string
      RETURNING
        VALUE(result) TYPE abap_bool.
  PROTECTED SECTION.
  PRIVATE SECTION.
ENDCLASS.



CLASS zcl_isogram IMPLEMENTATION.

  METHOD is_isogram.
    phrase = to_lower( phrase ).
    DATA(temp) = ''.
    result = abap_true.
    
    DO strlen( phrase ) TIMES.
      DATA(lv_index) = sy-index - 1.
      DATA(current_value) = phrase+lv_index(1).
     
      IF current_value CO temp.
        result = abap_false.
      ELSE.
        temp = temp && current_value.
      ENDIF.
    ENDDO.
  ENDMETHOD.

ENDCLASS.