/// Represents a magic number entry with offset, magic bytes, and description.
pub struct MagicEntry {
    pub offset: usize,
    pub magic: &'static [u8],
    pub description: &'static str,
}

pub fn get_magic_numbers() -> Vec<MagicEntry> {
    vec![
        // --- Image Files ---
        MagicEntry { offset: 0, magic: &[0x00, 0x00, 0x01, 0x00], description: "ICO icon, Windows icon" },
        MagicEntry { offset: 0, magic: &[0x00, 0x00, 0x02, 0x00], description: "CUR icon, Windows cursor" },
        MagicEntry { offset: 0, magic: &[0x01, 0x00, 0x09, 0x00], description: "Windows Metafile (WMF)" },
        MagicEntry { offset: 0, magic: &[0x20, 0x00, 0x00, 0x00, 0x4A, 0x46, 0x49, 0x46, 0x58, 0x58, 0x00], description: "JPEG File Interchange Format with extension data" },
        MagicEntry { offset: 0, magic: &[0x38, 0x42, 0x50, 0x53], description: "PSD image, Adobe Photoshop" },
        MagicEntry { offset: 0, magic: &[0x41, 0x43, 0x31, 0x30], description: "AutoCAD Drawing file (DWG)" },
        MagicEntry { offset: 0, magic: &[0x41, 0x49], description: "Adobe Illustrator Artwork" },
        MagicEntry { offset: 0, magic: &[0x42, 0x4D], description: "BMP image, a bitmap format" },
        MagicEntry { offset: 0, magic: &[0x47, 0x49, 0x46, 0x38, 0x37, 0x61], description: "GIF image data, version 87a" },
        MagicEntry { offset: 0, magic: &[0x47, 0x49, 0x46, 0x38, 0x39, 0x61], description: "GIF image data, version 89a" },
        MagicEntry { offset: 0, magic: &[0x49, 0x49, 0x2A, 0x00], description: "TIFF image data, little-endian" },
        MagicEntry { offset: 0, magic: &[0x49, 0x49, 0xBC], description: "Kodak Cineon image" },
        MagicEntry { offset: 0, magic: &[0x49, 0x54, 0x4F, 0x4C], description: "ITC (CMU WM) format" },
        MagicEntry { offset: 0, magic: &[0x4D, 0x4D, 0x00, 0x2A], description: "TIFF image data, big-endian" },
        MagicEntry { offset: 0, magic: &[0x52, 0x49, 0x46, 0x46], description: "RIFF container (AVI, WAV, WebP)" },
        MagicEntry { offset: 0, magic: &[0x53, 0x49, 0x4D, 0x50, 0x4C, 0x45], description: "FITS (Flexible Image Transport System)" },
        MagicEntry { offset: 0, magic: &[0x53, 0x56, 0x47, 0x20], description: "Scalable Vector Graphics (SVG)" },
        MagicEntry { offset: 0, magic: &[0x59, 0xA6, 0x6A, 0x95], description: "Sun Rasterfile" },
        MagicEntry { offset: 0, magic: &[0x67, 0x69, 0x6D, 0x70, 0x20, 0x78, 0x63, 0x66, 0x20], description: "GIMP image data" },
        MagicEntry { offset: 0, magic: &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A], description: "PNG image data, 8-bit depth" },
        MagicEntry { offset: 0, magic: &[0x8A, 0x4D, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A], description: "MNG, Multiple-image Network Graphics" },
        MagicEntry { offset: 0, magic: &[0x8B, 0x4A, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A], description: "JNG, JPEG Network Graphics" },
        MagicEntry { offset: 0, magic: &[0xFF, 0xD8, 0xFF], description: "JPEG image data (various standards)" },
        MagicEntry { offset: 0, magic: &[0xFF, 0xD8, 0xFF, 0xDB], description: "JPEG image data, JFIF/raw" },
        MagicEntry { offset: 0, magic: &[0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46], description: "JPEG image data, JFIF standard 1.01" },
        MagicEntry { offset: 0, magic: &[0xFF, 0xD8, 0xFF, 0xE1], description: "JPEG image data, Exif standard" },
        MagicEntry { offset: 0, magic: &[0x00, 0x00, 0x00, 0x0C, 0x6A, 0x50, 0x20, 0x20, 0x0D, 0x0A], description: "JPEG 2000 image data" },

        // --- Compressed and Archive Files ---
        MagicEntry { offset: 0, magic: &[0x1A], description: "Zoo archive data" },
        MagicEntry { offset: 0, magic: &[0x1F, 0x8B], description: "gzip compressed data" },
        MagicEntry { offset: 0, magic: &[0x1F, 0x9D], description: "compress'd data (Lempel-Ziv)" },
        MagicEntry { offset: 0, magic: &[0x1F, 0xA0], description: "compress'd data (LZH)" },
        MagicEntry { offset: 0, magic: &[0x21, 0x3C, 0x61, 0x72, 0x63, 0x68, 0x3E], description: "ar archive (Unix)" },
        MagicEntry { offset: 0, magic: &[0x21, 0x3C, 0x61, 0x72, 0x63, 0x68, 0x3E, 0x0A, 0x64, 0x65, 0x62, 0x69, 0x61, 0x6E, 0x2D, 0x62], description: "Debian binary package (.deb)" },
        MagicEntry { offset: 0, magic: &[0x28, 0xB5, 0x2F, 0xFD], description: "Zstandard compressed data" },
        MagicEntry { offset: 0, magic: &[0x30, 0x37, 0x30, 0x37, 0x30, 0x31], description: "cpio archive, new ASCII format" },
        MagicEntry { offset: 0, magic: &[0x30, 0x37, 0x30, 0x37, 0x30, 0x37], description: "cpio archive, old ASCII format" },
        MagicEntry { offset: 0, magic: &[0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C], description: "7-zip archive data" },
        MagicEntry { offset: 0, magic: &[0x42, 0x5A, 0x68], description: "bzip2 compressed data" },
        MagicEntry { offset: 0, magic: &[0x4D, 0x53, 0x43, 0x46], description: "Microsoft Cabinet file data" },
        MagicEntry { offset: 0, magic: &[0x50, 0x4B, 0x03, 0x04], description: "Zip archive data (PKZIP)" },
        MagicEntry { offset: 0, magic: &[0x50, 0x4B, 0x05, 0x06], description: "Zip archive data (empty)" },
        MagicEntry { offset: 0, magic: &[0x50, 0x4B, 0x07, 0x08], description: "Zip archive data (spanned)" },
        MagicEntry { offset: 0, magic: &[0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x00], description: "RAR archive data, v1.50" },
        MagicEntry { offset: 0, magic: &[0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x01, 0x00], description: "RAR archive data, v5.0+" },
        MagicEntry { offset: 0, magic: &[0x53, 0x5A, 0x44, 0x44, 0x88, 0xF0, 0x27, 0x33], description: "Microsoft compressed file in LZX format" },
        MagicEntry { offset: 0, magic: &[0x63, 0x70, 0x69, 0x6F], description: "cpio archive" },
        MagicEntry { offset: 0, magic: &[0x78, 0x01], description: "zlib compressed data, no compression/low" },
        MagicEntry { offset: 0, magic: &[0x78, 0x5E], description: "zlib compressed data, normal compression" },
        MagicEntry { offset: 0, magic: &[0x78, 0x9C], description: "zlib compressed data, default compression" },
        MagicEntry { offset: 0, magic: &[0x78, 0xDA], description: "zlib compressed data, best compression" },
        MagicEntry { offset: 0, magic: &[0xED, 0xAB, 0xEE, 0xDB], description: "RPM (Red Hat Package Manager) package" },
        MagicEntry { offset: 0, magic: &[0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00], description: "xz compressed data" },
        MagicEntry { offset: 0, magic: &[0x04, 0x22, 0x4D, 0x18], description: "LZ4 Frame format" },
        MagicEntry { offset: 257, magic: &[0x75, 0x73, 0x74, 0x61, 0x72, 0x00, 0x30, 0x30], description: "tar archive (POSIX ustar)" },
        MagicEntry { offset: 257, magic: &[0x75, 0x73, 0x74, 0x61, 0x72, 0x20, 0x20, 0x00], description: "tar archive (GNU)" },

        // --- Executable and System Files ---
        MagicEntry { offset: 0, magic: &[0x23, 0x21], description: "Script file with shebang (e.g., #!/bin/bash)" },
        MagicEntry { offset: 0, magic: &[0x43, 0x72, 0x32, 0x34], description: "Google Chrome extension/packaged app (.crx)" },
        MagicEntry { offset: 0, magic: &[0x4B, 0x44, 0x4D, 0x56], description: "VMDK (VMware virtual disk) file" },
        MagicEntry { offset: 0, magic: &[0x4C, 0x00, 0x00, 0x00, 0x01, 0x14, 0x02, 0x00], description: "Windows Shortcut file (.lnk)" },
        MagicEntry { offset: 0, magic: &[0x4D, 0x5A], description: "DOS MZ executable, for MS-DOS, OS/2 or MS Windows" },
        MagicEntry { offset: 0, magic: &[0x50, 0x45, 0x00, 0x00], description: "PE32 executable (Windows)" }, // Often follows an MZ header at a variable offset
        MagicEntry { offset: 0, magic: &[0x64, 0x65, 0x78, 0x0A, 0x30, 0x33, 0x35, 0x00], description: "Dalvik Executable format (.dex)" },
        MagicEntry { offset: 0, magic: &[0x7F, 0x45, 0x4C, 0x46], description: "ELF executable or shared object, for Linux/Unix" },
        MagicEntry { offset: 0, magic: &[0xCA, 0xFE, 0xBA, 0xBE], description: "Java class file" },
        MagicEntry { offset: 0, magic: &[0xCE, 0xFA, 0xED, 0xFE], description: "Mach-O executable (32-bit, little-endian)" },
        MagicEntry { offset: 0, magic: &[0xCF, 0xFA, 0xED, 0xFE], description: "Mach-O executable (64-bit, little-endian)" },
        MagicEntry { offset: 0, magic: &[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1], description: "Microsoft Office/Compound File Binary Format (doc, xls, ppt, msi, etc.)" },
        MagicEntry { offset: 0, magic: &[0xFE, 0xED, 0xFA, 0xCE], description: "Mach-O executable (32-bit, big-endian)" },
        MagicEntry { offset: 0, magic: &[0xFE, 0xED, 0xFA, 0xCF], description: "Mach-O executable (64-bit, big-endian)" },

        // --- Document Formats ---
        MagicEntry { offset: 0, magic: &[0x25, 0x21, 0x50, 0x53], description: "PostScript document" },
        MagicEntry { offset: 0, magic: &[0x25, 0x50, 0x44, 0x46], description: "PDF document" },
        MagicEntry { offset: 0, magic: &[0x3C, 0x21, 0x44, 0x4F, 0x43, 0x54, 0x59, 0x50, 0x45, 0x20, 0x68, 0x74, 0x6D, 0x6C], description: "HTML document" },
        MagicEntry { offset: 0, magic: &[0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20], description: "XML document" },
        MagicEntry { offset: 0, magic: &[0x4F, 0x70, 0x65, 0x6E, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x20, 0x54, 0x65, 0x78, 0x74], description: "OpenDocument Text (ODT)" },
        MagicEntry { offset: 0, magic: &[0x7B, 0x5C, 0x72, 0x74, 0x66, 0x31], description: "Rich Text Format (RTF) data" },

        // --- Audio and Video Files ---
        MagicEntry { offset: 0, magic: &[0x00, 0x00, 0x00, 0x14, 0x66, 0x74, 0x79, 0x70, 0x69, 0x73, 0x6F, 0x6D], description: "MP4 video file (ISO base media file format)" },
        MagicEntry { offset: 0, magic: &[0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70, 0x33, 0x67], description: "3GPP multimedia file (.3gp)" },
        MagicEntry { offset: 0, magic: &[0x00, 0x00, 0x01, 0xB3], description: "MPEG-1 video" },
        MagicEntry { offset: 0, magic: &[0x00, 0x00, 0x01, 0xBA], description: "MPEG-PS (Program Stream)" },
        MagicEntry { offset: 0, magic: &[0x1A, 0x45, 0xDF, 0xA3], description: "Matroska (MKV) data container (WebM, etc.)" },
        MagicEntry { offset: 0, magic: &[0x2E, 0x72, 0x61, 0xFD], description: "RealMedia file (.ra)" },
        MagicEntry { offset: 0, magic: &[0x2E, 0x73, 0x6E, 0x64], description: "NeXT/Sun Audio file (.au)" },
        MagicEntry { offset: 0, magic: &[0x30, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11, 0xA6, 0xD9, 0x00, 0xAA, 0x00, 0x62, 0xCE, 0x6C], description: "ASF/WMV/WMA file" },
        MagicEntry { offset: 0, magic: &[0x46, 0x4C, 0x56, 0x01], description: "FLV (Flash Video) data" },
        MagicEntry { offset: 0, magic: &[0x47], description: "MPEG-TS (Transport Stream)" },
        MagicEntry { offset: 0, magic: &[0x49, 0x44, 0x33], description: "MP3 audio with ID3v2 tag" },
        MagicEntry { offset: 0, magic: &[0x4F, 0x67, 0x67, 0x53], description: "Ogg data container (Vorbis, Theora, etc.)" },
        MagicEntry { offset: 0, magic: &[0x4D, 0x54, 0x68, 0x64], description: "MIDI (Musical Instrument Digital Interface) data" },
        MagicEntry { offset: 0, magic: &[0x52, 0x49, 0x46, 0x58], description: "RIFX (big-endian) data container" },
        MagicEntry { offset: 0, magic: &[0x66, 0x4C, 0x61, 0x43], description: "FLAC (Free Lossless Audio Codec) data" },
        MagicEntry { offset: 0, magic: &[0xFF, 0xF1], description: "MPEG-4 AAC ADTS file" },
        MagicEntry { offset: 0, magic: &[0xFF, 0xF2], description: "MP3 audio file (MPEG ADTS, layer III, v1, no CRC)" },
        MagicEntry { offset: 0, magic: &[0xFF, 0xF3], description: "MP3 audio file (MPEG ADTS, layer III, v1, with CRC)" },
        MagicEntry { offset: 0, magic: &[0xFF, 0xFB], description: "MP3 audio file (no ID3 tag, MPEG-1 Layer 3)" },
        MagicEntry { offset: 8, magic: b"AVI " as &[u8], description: "AVI video file (within RIFF)" },
        MagicEntry { offset: 8, magic: b"WAVE" as &[u8], description: "WAVE audio file (within RIFF/RIFX)" },
        MagicEntry { offset: 0, magic: b"moov" as &[u8], description: "MOV video file (QuickTime Movie) 'moov' atom" },

        // --- Database Files ---
        MagicEntry { offset: 0, magic: &[0x00, 0x01, 0x00, 0x00, 0x53, 0x74, 0x61, 0x6E, 0x64, 0x61, 0x72, 0x64, 0x20, 0x41, 0x43, 0x45, 0x20, 0x44, 0x42], description: "Microsoft Access 2007+ Database (.accdb)" },
        MagicEntry { offset: 0, magic: &[0x00, 0x01, 0x00, 0x00, 0x53, 0x74, 0x61, 0x6E, 0x64, 0x61, 0x72, 0x64, 0x20, 0x4A, 0x65, 0x74, 0x20, 0x44, 0x42], description: "Microsoft Access 2000/2003 Database (.mdb)" },
        MagicEntry { offset: 0, magic: &[0x53, 0x51, 0x4C, 0x69, 0x74, 0x65, 0x20, 0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x20, 0x33, 0x00], description: "SQLite 3 database file" },

        // --- Font Files ---
        MagicEntry { offset: 0, magic: &[0x00, 0x01, 0x00, 0x00, 0x00], description: "TrueType Font file (.ttf)" },
        MagicEntry { offset: 0, magic: &[0x4F, 0x54, 0x54, 0x4F], description: "OpenType Font file (.otf)" },
        MagicEntry { offset: 0, magic: &[0x74, 0x74, 0x63, 0x66], description: "TrueType Font Collection (.ttc)" },
        MagicEntry { offset: 0, magic: &[0x77, 0x4F, 0x46, 0x32], description: "Web Open Font Format 2.0 (.woff2)" },
        MagicEntry { offset: 0, magic: &[0x77, 0x4F, 0x46, 0x46], description: "Web Open Font Format 1.0 (.woff)" },

        // --- Disk Images and Filesystems ---
        MagicEntry { offset: 0, magic: &[0x43, 0x44, 0x30, 0x30, 0x31], description: "ISO-9660 CD/DVD image (.iso)" },
        MagicEntry { offset: 1024, magic: &[0x48, 0x2B, 0x00, 0x00], description: "HFS+ filesystem data" },
        MagicEntry { offset: 3, magic: &[0x4E, 0x54, 0x46, 0x53, 0x20, 0x20, 0x20, 0x20], description: "NTFS filesystem data" },
        MagicEntry { offset: 1080, magic: &[0x53, 0xEF], description: "ext2/ext3/ext4 filesystem data" },
        MagicEntry { offset: 0, magic: &[0x72, 0x65, 0x69, 0x73, 0x65, 0x72], description: "ReiserFS filesystem data" },
        MagicEntry { offset: 0, magic: &[0xEB, 0x3C, 0x90], description: "FAT12/FAT16 filesystem data" },
        MagicEntry { offset: 0, magic: &[0xEB, 0x52, 0x90], description: "FAT32 filesystem data" },
        MagicEntry { offset: 0, magic: &[0xEB, 0x76, 0x90], description: "exFAT filesystem data" },

        // --- Miscellaneous & Less Common ---
        MagicEntry { offset: 0, magic: &[0x21, 0x42, 0x44, 0x4E], description: "Outlook Personal Storage Table (.pst)" },
        MagicEntry { offset: 0, magic: &[0x30, 0x82], description: "DER encoded security certificate" },
        MagicEntry { offset: 128, magic: &[0x44, 0x43, 0x4D, 0x49], description: "DICOM Medical file format" },
        MagicEntry { offset: 0, magic: &[0x47, 0x4F, 0x44, 0x4F, 0x54, 0x45, 0x4E, 0x43], description: "Godot Engine encrypted script (.gde)" },
        MagicEntry { offset: 0, magic: &[0x50, 0x4D, 0x4F, 0x43, 0x43, 0x4D, 0x4F, 0x43], description: "Windows Performance Monitor counter file (.pmc)" },
        MagicEntry { offset: 0, magic: &[0x55, 0x6E, 0x69, 0x74, 0x79, 0x46, 0x53], description: "Unity game data archive (UnityFS)" },
        MagicEntry { offset: 0, magic: &[0x7b, 0x0d, 0x0a, 0x20, 0x20, 0x22], description: "JSON file" }, // Heuristic, not a standard magic number
        MagicEntry { offset: 0, magic: b"-----BEGIN CERTIFICATE-----" as &[u8], description: "PEM security certificate" },
        MagicEntry { offset: 4, magic: b"regf" as &[u8], description: "Windows Registry hive file" },
    ]
}