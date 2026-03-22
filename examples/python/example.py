#!/usr/bin/env python3
"""
BrepRs Python Example

This example demonstrates how to use BrepRs Python bindings.
"""

from breprs_python import I18n, Point, Shape

def main():
    print("BrepRs Python Example")
    print("=" * 50)

    # Initialize i18n with automatic language detection
    I18n.init()
    print(f"Current language: {I18n.current_language()}")

    # Get available languages
    languages = I18n.available_languages()
    print(f"Available languages: {', '.join(languages)}")

    # Set language to English
    I18n.set_language("en")
    print(f"Language set to: {I18n.current_language()}")

    # Translate some messages
    print("\nTranslations (English):")
    print(f"  ErrorUnknown: {I18n.translate('ErrorUnknown')}")
    print(f"  LabelFile: {I18n.translate('LabelFile')}")
    print(f"  OpBooleanFuse: {I18n.translate('OpBooleanFuse')}")

    # Set language to Simplified Chinese
    I18n.set_language("zh-CN")
    print(f"\nLanguage set to: {I18n.current_language()}")

    print("\nTranslations (Simplified Chinese):")
    print(f"  ErrorUnknown: {I18n.translate('ErrorUnknown')}")
    print(f"  LabelFile: {I18n.translate('LabelFile')}")
    print(f"  OpBooleanFuse: {I18n.translate('OpBooleanFuse')}")

    # Create a point
    point = Point(1.0, 2.0, 3.0)
    print(f"\nCreated point: {point}")

    # Create a shape
    shape = Shape()
    print(f"Created shape: {shape}")
    print(f"Shape type: {shape.shape_type()}")
    print(f"Shape is null: {shape.is_null()}")

    print("\nExample completed successfully!")

if __name__ == "__main__":
    main()
