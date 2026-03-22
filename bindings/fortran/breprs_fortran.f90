! Fortran bindings for BrepRs
!
! This module provides Fortran bindings for BrepRs via C FFI.
!
! Note: Fortran bindings are provided through the C FFI interface.
! See bindings/cpp/lib.rs for the C implementation.

module breprs_fortran
    use iso_c_binding, only: c_char, c_double, c_int, c_ptr, c_null_ptr, c_f_pointer

    implicit none

    private
    public :: breprs_i18n_init, breprs_i18n_set_language, &
             breprs_i18n_current_language, breprs_i18n_translate, &
             breprs_point_new, breprs_point_get_coords, breprs_point_free, &
             breprs_shape_new, breprs_shape_is_null, breprs_shape_free, &
             breprs_free_string

    interface

        ! Result codes for BrepRs operations
        integer, parameter :: BREPRS_SUCCESS = 0
        integer, parameter :: BREPRS_ERROR = -1
        integer, parameter :: BREPRS_INVALID_INPUT = -2
        integer, parameter :: BREPRS_NULL_POINTER = -3
        integer, parameter :: BREPRS_OUT_OF_MEMORY = -4

        ! Initialize BrepRs i18n with automatic system language detection
        subroutine breprs_i18n_init() bind(c, name="breprs_i18n_init")
        end subroutine breprs_i18n_init

        ! Set the current language
        function breprs_i18n_set_language(lang_code) result(res) bind(c, name="breprs_i18n_set_language")
            import :: c_char, c_int
            character(kind=c_char), intent(in) :: lang_code(*)
            integer(kind=c_int) :: res
        end function breprs_i18n_set_language

        ! Get the current language code
        function breprs_i18n_current_language() result(lang_code) bind(c, name="breprs_i18n_current_language")
            import :: c_ptr
            type(c_ptr) :: lang_code
        end function breprs_i18n_current_language

        ! Translate a message key
        function breprs_i18n_translate(key) result(translation) bind(c, name="breprs_i18n_translate")
            import :: c_char, c_ptr
            character(kind=c_char), intent(in) :: key(*)
            type(c_ptr) :: translation
        end function breprs_i18n_translate

        ! Create a new point
        function breprs_point_new(x, y, z) result(point) bind(c, name="breprs_point_new")
            import :: c_double, c_ptr
            real(kind=c_double), value :: x, y, z
            type(c_ptr) :: point
        end function breprs_point_new

        ! Get point coordinates
        function breprs_point_get_coords(point, x, y, z) result(res) bind(c, name="breprs_point_get_coords")
            import :: c_double, c_int, c_ptr
            type(c_ptr), value :: point
            real(kind=c_double), intent(out) :: x, y, z
            integer(kind=c_int) :: res
        end function breprs_point_get_coords

        ! Free a point
        subroutine breprs_point_free(point) bind(c, name="breprs_point_free")
            import :: c_ptr
            type(c_ptr), value :: point
        end subroutine breprs_point_free

        ! Create a new shape
        function breprs_shape_new() result(shape) bind(c, name="breprs_shape_new")
            import :: c_ptr
            type(c_ptr) :: shape
        end function breprs_shape_new

        ! Check if shape is null
        function breprs_shape_is_null(shape) result(is_null) bind(c, name="breprs_shape_is_null")
            import :: c_int, c_ptr
            type(c_ptr), value :: shape
            integer(kind=c_int) :: is_null
        end function breprs_shape_is_null

        ! Free a shape
        subroutine breprs_shape_free(shape) bind(c, name="breprs_shape_free")
            import :: c_ptr
            type(c_ptr), value :: shape
        end subroutine breprs_shape_free

        ! Free a string allocated by BrepRs
        subroutine breprs_free_string(str) bind(c, name="breprs_free_string")
            import :: c_ptr
            type(c_ptr), value :: str
        end subroutine breprs_free_string

    end interface

contains

    ! Helper function to convert C string to Fortran string
    function c_to_f_string(c_str) result(f_str)
        type(c_ptr), intent(in) :: c_str
        character(len=:), allocatable :: f_str
        character(kind=c_char), pointer :: c_ptr
        integer :: i, len

        if (.not. c_associated(c_str)) then
            f_str = ""
            return
        end if

        call c_f_pointer(c_str, c_ptr)

        len = 0
        do while (c_ptr(len+1) /= c_null_char())
            len = len + 1
        end do

        allocate(character(len=len) :: f_str)
        do i = 1, len
            f_str(i:i) = c_ptr(i)
        end do
    end function c_to_f_string

    ! Helper function to convert Fortran string to C string
    function f_to_c_string(f_str) result(c_str)
        character(len=*), intent(in) :: f_str
        character(len=len_trim(f_str)+1, kind=c_char) :: c_str
        integer :: i

        do i = 1, len_trim(f_str)
            c_str(i:i) = f_str(i:i)
        end do
        c_str(len_trim(f_str)+1:len_trim(f_str)+1) = c_null_char()
    end function f_to_c_string

end module breprs_fortran
