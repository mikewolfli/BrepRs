# Documentation BrepRs

Bienvenue dans la documentation BrepRs ! BrepRs est une bibliothèque de représentation de frontière (BRep) moderne et performante écrite en Rust, conçue pour les applications CAO/CAE/CAM.

## Qu'est-ce que BrepRs ?

BrepRs est une bibliothèque de représentation de frontière (BRep) qui fournit :

- **Opérations géométriques et topologiques** haute performance
- **Prise en charge multilingue** via des liaisons de langage
- **API complète** pour la modélisation 3D
- **Compatibilité multiplateforme**
- **Support WebAssembly** pour les applications navigateur

## Caractéristiques principales

### Caractéristiques centrales

- **Opérations booléennes** (fusion, découpe, intersection, section)
- **Modélisation de surfaces** (plans, cylindres, sphères, cônes, tores)
- **Opérations topologiques** (manipulation de sommets, arêtes, faces, coques, solides)
- **Génération de maillage** et manipulation
- **Entrée/sortie de fichiers** (STEP, IGES, STL, OBJ, glTF)
- **Support d'internationalisation**

### Liaisons de langage

BrepRs fournit des liaisons pour plusieurs langages de programmation :

- **Python** - En utilisant PyO3
- **C/C++** - En utilisant FFI
- **Fortran** - En utilisant C FFI
- **Java** - En utilisant JNI
- **Node.js** - En utilisant NAPI-RS
- **PHP** - En utilisant ext-php-rs

### Performances

- **Basé sur Rust** pour des performances maximales
- **Traitement parallèle** avec Rayon
- **Conception sécurisée en mémoire**
- **Opérations sans copie** lorsque possible
- **Algorithmes optimisés** pour les opérations courantes

## Cas d'utilisation

BrepRs convient à un large éventail d'applications :

- **Systèmes CAO** - Conception assistée par ordinateur
- **Systèmes CAM** - Fabrication assistée par ordinateur
- **Systèmes CAE** - Ingénierie assistée par ordinateur
- **Visualisation 3D** - Rendu et simulation
- **Traitement de géométrie** - Analyse et manipulation
- **Fabrication additive** - Préparation pour impression 3D
- **Robotique** - Planification de trajectoire et détection de collision

## Prise en main

Pour commencer avec BrepRs, consultez la section [Prise en main](fr/getting-started.md), qui comprend des instructions d'installation et des guides de démarrage rapide.

## Référence API

La section [Référence API](fr/api-reference.md) fournit une documentation détaillée de toutes les API BrepRs, y compris les opérations de géométrie, de topologie, de modélisation et d'entrée/sortie.

## Exemples

La section [Exemples](fr/examples.md) contient des exemples de code démontrant les cas d'utilisation courants et les flux de travail.

## Contribution

BrepRs est un projet open source. Nous accueillons les contributions de la communauté. Veuillez consulter la section [Contribution](fr/contributing.md) pour plus d'informations.

## Licence

BrepRs est disponible sous licence MIT ou Apache 2.0. Voir la section [Licence](fr/license.md) pour plus de détails.
