/*
 * BrepRs C++ Example
 *
 * This example demonstrates how to use BrepRs C++ bindings.
 */

#include <iostream>
#include <cstring>
#include "breprs.h"

int main() {
    std::cout << "BrepRs C++ Example" << std::endl;
    std::cout << "==================" << std::endl;

    // Initialize i18n with automatic language detection
    breprs_i18n_init();
    char* current_lang = breprs_i18n_current_language();
    std::cout << "Current language: " << current_lang << std::endl;
    breprs_free_string(current_lang);

    // Set language to English
    BrepResult result = breprs_i18n_set_language("en");
    if (result == BREPRS_SUCCESS) {
        std::cout << "Language set to English" << std::endl;
    }

    // Translate some messages
    std::cout << "\nTranslations (English):" << std::endl;

    char* error_unknown = breprs_i18n_translate("ErrorUnknown");
    std::cout << "  ErrorUnknown: " << error_unknown << std::endl;
    breprs_free_string(error_unknown);

    char* label_file = breprs_i18n_translate("LabelFile");
    std::cout << "  LabelFile: " << label_file << std::endl;
    breprs_free_string(label_file);

    char* op_boolean_fuse = breprs_i18n_translate("OpBooleanFuse");
    std::cout << "  OpBooleanFuse: " << op_boolean_fuse << std::endl;
    breprs_free_string(op_boolean_fuse);

    // Set language to Simplified Chinese
    result = breprs_i18n_set_language("zh-CN");
    if (result == BREPRS_SUCCESS) {
        std::cout << "\nLanguage set to Simplified Chinese" << std::endl;
    }

    std::cout << "\nTranslations (Simplified Chinese):" << std::endl;

    error_unknown = breprs_i18n_translate("ErrorUnknown");
    std::cout << "  ErrorUnknown: " << error_unknown << std::endl;
    breprs_free_string(error_unknown);

    label_file = breprs_i18n_translate("LabelFile");
    std::cout << "  LabelFile: " << label_file << std::endl;
    breprs_free_string(label_file);

    op_boolean_fuse = breprs_i18n_translate("OpBooleanFuse");
    std::cout << "  OpBooleanFuse: " << op_boolean_fuse << std::endl;
    breprs_free_string(op_boolean_fuse);

    // Create a point
    BrepPoint* point = breprs_point_new(1.0, 2.0, 3.0);
    if (point) {
        double x, y, z;
        result = breprs_point_get_coords(point, &x, &y, &z);
        if (result == BREPRS_SUCCESS) {
            std::cout << "\nCreated point: (" << x << ", " << y << ", " << z << ")" << std::endl;
        }
        breprs_point_free(point);
    }

    // Create a shape
    BrepShape* shape = breprs_shape_new();
    if (shape) {
        int is_null = breprs_shape_is_null(shape);
        std::cout << "Created shape" << std::endl;
        std::cout << "Shape is null: " << (is_null ? "true" : "false") << std::endl;
        breprs_shape_free(shape);
    }

    std::cout << "\nExample completed successfully!" << std::endl;
    return 0;
}
