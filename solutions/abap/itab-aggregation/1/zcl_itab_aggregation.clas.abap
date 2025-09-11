CLASS zcl_itab_aggregation DEFINITION
  PUBLIC
  FINAL
  CREATE PUBLIC .

  PUBLIC SECTION.
    TYPES group TYPE c LENGTH 1.
    TYPES: BEGIN OF initial_numbers_type,
             group  TYPE group,
             number TYPE i,
           END OF initial_numbers_type,
           initial_numbers TYPE STANDARD TABLE OF initial_numbers_type WITH EMPTY KEY.

    TYPES: BEGIN OF aggregated_data_type,
             group   TYPE group,
             count   TYPE i,
             sum     TYPE i,
             min     TYPE i,
             max     TYPE i,
             average TYPE f,
           END OF aggregated_data_type,
           aggregated_data TYPE STANDARD TABLE OF aggregated_data_type WITH EMPTY KEY.

    METHODS perform_aggregation
      IMPORTING
        initial_numbers        TYPE initial_numbers
      RETURNING
        VALUE(aggregated_data) TYPE aggregated_data.
  PROTECTED SECTION.
  PRIVATE SECTION.

ENDCLASS.



CLASS zcl_itab_aggregation IMPLEMENTATION.
  METHOD perform_aggregation.
    LOOP AT initial_numbers ASSIGNING FIELD-SYMBOL(<fs_inl>).
      IF line_exists( aggregated_data[ group = <fs_inl>-group ] ).
        DATA(index) = line_index( aggregated_data[ group = <fs_inl>-group ] ).
        
        aggregated_data[ index ]-count = aggregated_data[ index ]-count + 1.
        aggregated_data[ index ]-sum = aggregated_data[ index ]-sum + <fs_inl>-number.
        aggregated_data[ index ]-min = nmin( val1 = aggregated_data[ index ]-min val2 = <fs_inl>-number ).
        aggregated_data[ index ]-max = nmax( val1 = aggregated_data[ index ]-max val2 = <fs_inl>-number ).
        aggregated_data[ index ]-average = aggregated_data[ index ]-sum / aggregated_data[ index ]-count.
        
        ELSE.
          APPEND VALUE #( group = <fs_inl>-group count = 1 sum = <fs_inl>-number min = <fs_inl>-number max = <fs_inl>-number average = <fs_inl>-number ) TO aggregated_data.
      ENDIF.
    ENDLOOP.

  ENDMETHOD.

ENDCLASS.