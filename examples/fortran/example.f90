! BrepRs Fortran Example
!
! This example demonstrates how to use BrepRs Fortran bindings.

program breprs_fortran_example
    use breprs_fortran
    implicit none

    character(len=:), allocatable :: current_lang, error_unknown, label_file, op_boolean_fuse
    integer(kind=c_int) :: result, is_null
    type(c_ptr) :: c_str
    real(kind=c_double) :: x, y, z
    type(c_ptr) :: point, shape

    print *, "BrepRs Fortran Example"
    print *, "======================="

    ! Initialize i18n with automatic language detection
    call breprs_i18n_init()
    c_str = breprs_i18n_current_language()
    current_lang = c_to_f_string(c_str)
    print *, "Current language: ", trim(current_lang)
    call breprs_free_string(c_str)

    ! Set language to English
    result = breprs_i18n_set_language("en" // c_null_char())
    if (result == BREPRS_SUCCESS) then
        print *, "Language set to English"
    end if

    ! Translate some messages
    print *, ""
    print *, "Translations (English):"

    c_str = breprs_i18n_translate("ErrorUnknown" // c_null_char())
    error_unknown = c_to_f_string(c_str)
    print *, "  ErrorUnknown: ", trim(error_unknown)
    call breprs_free_string(c_str)

    c_str = breprs_i18n_translate("LabelFile" // c_null_char())
    label_file = c_to_f_string(c_str)
    print *, "  LabelFile: ", trim(label_file)
    call breprs_free_string(c_str)

    c_str = breprs_i18n_translate("OpBooleanFuse" // c_null_char())
    op_boolean_fuse = c_to_f_string(c_str)
    print *, "  OpBooleanFuse: ", trim(op_boolean_fuse)
    call breprs_free_string(c_str)

    ! Set language to Simplified Chinese
    result = breprs_i18n_set_language("zh-CN" // c_null_char())
    if (result == BREPRS_SUCCESS) then
        print *, ""
        print *, "Language set to Simplified Chinese"
    end if

    print *, ""
    print *, "Translations (Simplified Chinese):"

    c_str = breprs_i18n_translate("ErrorUnknown" // c_null_char())
    error_unknown = c_to_f_string(c_str)
    print *, "  ErrorUnknown: ", trim(error_unknown)
    call breprs_free_string(c_str)

    c_str = breprs_i18n_translate("LabelFile" // c_null_char())
    label_file = c_to_f_string(c_str)
    print *, "  LabelFile: ", trim(label_file)
    call breprs_free_string(c_str)

    c_str = breprs_i18n_translate("OpBooleanFuse" // c_null_char())
    op_boolean_fuse = c_to_f_string(c_str)
    print *, "  OpBooleanFuse: ", trim(op_boolean_fuse)
    call breprs_free_string(c_str)

    ! Create a point
    point = breprs_point_new(1.0d0, 2.0d0, 3.0d0)
    if (c_associated(point)) then
        result = breprs_point_get_coords(point, x, y, z)
        if (result == BREPRS_SUCCESS) then
            print *, ""
            print *, "Created point: (", x, ", ", y, ", ", z, ")"
        end if
        call breprs_point_free(point)
    end if

    ! Create a shape
    shape = breprs_shape_new()
    if (c_associated(shape)) then
        is_null = breprs_shape_is_null(shape)
        print *, ""
        print *, "Created shape"
        print *, "Shape is null: ", is_null
        call breprs_shape_free(shape)
    end if

    print *, ""
    print *, "Example completed successfully!"

end program breprs_fortran_example
