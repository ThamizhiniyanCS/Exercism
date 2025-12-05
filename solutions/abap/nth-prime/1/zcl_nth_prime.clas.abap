CLASS zcl_nth_prime DEFINITION.

  PUBLIC SECTION.
    METHODS prime
      IMPORTING
        input           TYPE i
      RETURNING
        VALUE(result_0) TYPE i
      RAISING
        cx_parameter_invalid.

    METHODS is_prime
      IMPORTING
        VALUE(number)   TYPE i
      RETURNING
        VALUE(result_1) TYPE abap_bool.

  PROTECTED SECTION.
  PRIVATE SECTION.

ENDCLASS.


CLASS zcl_nth_prime IMPLEMENTATION.
  METHOD prime.
    IF input = 0.
      RAISE EXCEPTION TYPE cx_parameter_invalid.
    ENDIF.

    DATA(counter_0) = 1.

    WHILE counter_0 <> ( input + 1 ).
      IF is_prime( sy-index ) = abap_true.
        result_0 = sy-index.
        counter_0 = counter_0 + 1.
      ENDIF.
    ENDWHILE.
  ENDMETHOD.

  METHOD is_prime.
    result_1 = abap_true.

    IF number <= 1.
      result_1 = abap_false.
    ELSE.
      DATA(limit) = number DIV 2.
      DATA(counter_1) = 2.

      WHILE counter_1 <= limit.
        IF ( number MOD counter_1 ) = 0.
          result_1 = abap_false.
        ENDIF.

        counter_1 = counter_1 + 1.
      ENDWHILE.
    ENDIF.
  ENDMETHOD.
ENDCLASS.