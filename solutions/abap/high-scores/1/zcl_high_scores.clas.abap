CLASS zcl_high_scores DEFINITION
  PUBLIC
  FINAL
  CREATE PUBLIC .

  PUBLIC SECTION.
    TYPES integertab TYPE STANDARD TABLE OF i WITH EMPTY KEY.
    METHODS constructor
      IMPORTING
        scores TYPE integertab.

    METHODS list_scores
      RETURNING
        VALUE(result) TYPE integertab.

    METHODS latest
      RETURNING
        VALUE(result) TYPE i.

    METHODS personalbest
      RETURNING
        VALUE(result) TYPE i.

    METHODS personaltopthree
      RETURNING
        VALUE(result) TYPE integertab.
  PROTECTED SECTION.
  PRIVATE SECTION.
    DATA scores_list TYPE integertab.

ENDCLASS.


CLASS zcl_high_scores IMPLEMENTATION.

  METHOD constructor.
    me->scores_list = scores.
  ENDMETHOD.

  METHOD list_scores.
    result = me->scores_list.
  ENDMETHOD.

  METHOD latest.
    result = me->scores_list[ lines( scores_list ) ].
  ENDMETHOD.

  METHOD personalbest.
    result = REDUCE #( 
                       INIT max = 0
                       FOR value IN me->scores_list
                       NEXT max = COND #( WHEN value > max THEN value ELSE max )
                     ).
  ENDMETHOD.

  METHOD personaltopthree.
    SORT me->scores_list DESCENDING.

    IF lines( me->scores_list ) <= 3.
      result = me->scores_list.

      ELSE.
        result = VALUE #(
          ( me->scores_list[ 1 ] )
          ( me->scores_list[ 2 ] )
          ( me->scores_list[ 3 ] )
        ).
    ENDIF.
  ENDMETHOD.


ENDCLASS.
