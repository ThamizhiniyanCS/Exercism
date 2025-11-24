CLASS zcl_clock DEFINITION
  PUBLIC
  CREATE PUBLIC.

  PUBLIC SECTION.

    METHODS constructor
      IMPORTING
        !hours   TYPE i
        !minutes TYPE i DEFAULT 0.
    METHODS get
      RETURNING
        VALUE(result) TYPE string.
    METHODS add
      IMPORTING
        !minutes TYPE i.
    METHODS sub
      IMPORTING
        !minutes TYPE i.

  PRIVATE SECTION.  
    " clock is basically total number of seconds
    DATA clock TYPE t.
    
ENDCLASS.



CLASS zcl_clock IMPLEMENTATION.

  METHOD add.
    " Converting the minutes to seconds first and adding it to the clock
    clock = clock + minutes * 60.
  ENDMETHOD.

  METHOD constructor.
    " Converting hours to seconds and minutes to seconds and adding it to get the total number of seconds
    clock = hours * 60  * 60 + minutes * 60.
  ENDMETHOD.

  METHOD get.
    " https://github.com/SAP-samples/abap-cheat-sheets/blob/main/23_Date_and_Time.md#excursions
    result = |{ clock TIME = ISO }|.
    " https://github.com/SAP-samples/abap-cheat-sheets/blob/main/23_Date_and_Time.md#accessing-time-values
    " returning only hh:mm
    result = result(5).
  ENDMETHOD.

  METHOD sub.
    " Converting the minutes to seconds first and subtracting it from the clock
    clock = clock - minutes * 60.
  ENDMETHOD.
ENDCLASS.
