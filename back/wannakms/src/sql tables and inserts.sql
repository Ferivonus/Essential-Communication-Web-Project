-- Drop tables if they exist
DROP TABLE IF EXISTS messages_send_to_user_client_files;
DROP TABLE IF EXISTS messages_send_to_other_client_files;
DROP TABLE IF EXISTS sent_images;
DROP TABLE IF EXISTS sent_gifs;
DROP TABLE IF EXISTS sent_documents;
DROP TABLE IF EXISTS messages_send_to_user_client;
DROP TABLE IF EXISTS messages_send_to_other_client;
DROP TABLE IF EXISTS user_server_allowed_message_list;
DROP TABLE IF EXISTS other_message_server_user_want_connect;
DROP TABLE IF EXISTS user_server_phone_numbers;
DROP TABLE IF EXISTS other_server_phone_numbers;
DROP TABLE IF EXISTS user_server_social_media;
DROP TABLE IF EXISTS other_server_social_media;
DROP TABLE IF EXISTS other_server_people_extra_info;
DROP TABLE IF EXISTS user_server_people_extra_info;
DROP TABLE IF EXISTS user_server_profile_pictures;
DROP TABLE IF EXISTS other_server_profile_pictures;
DROP TABLE IF EXISTS form_pages;
DROP TABLE IF EXISTS user_server_people;
DROP TABLE IF EXISTS other_server_people;
DROP TABLE IF EXISTS sent_files;
-- Assuming this is also a table, as it's referenced by foreign keys.
DROP TABLE IF EXISTS user_server_connection_keys;
DROP TABLE IF EXISTS other_server_connection_keys;
CREATE TABLE IF NOT EXISTS form_pages (
    id INT AUTO_INCREMENT PRIMARY KEY,
    -- Unique identifier for each form page
    form_name VARCHAR(255) NOT NULL,
    -- Name of the form
    page_url VARCHAR(255) NOT NULL UNIQUE,
    -- URL or route of the page containing the form
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    -- Timestamp when the form page was created
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    -- Timestamp when the form page was last updated
    is_active BOOLEAN DEFAULT TRUE -- Indicates if the form page is currently active
);
-- Table for storing connection information for people on your server
CREATE TABLE IF NOT EXISTS user_server_connection_keys (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_secret_key VARCHAR(225) NOT NULL,
    other_side_secret_key VARCHAR(225) NOT NULL,
    is_connection_possible BOOLEAN DEFAULT(false),
    connection_link VARCHAR(225) NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
-- Table for storing connection information for people on the other server
CREATE TABLE IF NOT EXISTS other_server_connection_keys (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_secret_key VARCHAR(225) NOT NULL,
    other_side_secret_key VARCHAR(225) NOT NULL,
    is_connection_possible BOOLEAN DEFAULT(false),
    connection_link VARCHAR(225) NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
-- Updated table for storing people on your server (main table)
CREATE TABLE IF NOT EXISTS user_server_people (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nick VARCHAR(255) NOT NULL,
    connection_id INT,
    -- Foreign key linking to user_server_connection_keys
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (connection_id) REFERENCES user_server_connection_keys(id) ON DELETE CASCADE
);
-- Updated table for storing people on the other server (main table)
CREATE TABLE IF NOT EXISTS other_server_people (
    id INT AUTO_INCREMENT PRIMARY KEY,
    nick VARCHAR(255) NOT NULL,
    connection_id INT,
    -- Foreign key linking to other_server_connection_keys
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (connection_id) REFERENCES other_server_connection_keys(id) ON DELETE CASCADE
);
ALTER TABLE user_server_people AUTO_INCREMENT = 1;
ALTER TABLE other_server_people AUTO_INCREMENT = 1;
-- Extra info for user server people (linked by id)
CREATE TABLE IF NOT EXISTS user_server_people_extra_info (
    user_id INT NOT NULL,
    -- Use id as foreign key
    name VARCHAR(255),
    surname VARCHAR(255),
    age INT,
    location VARCHAR(255),
    occupation VARCHAR(255),
    extra_info TEXT,
    FOREIGN KEY (user_id) REFERENCES user_server_people (id) ON DELETE CASCADE
);
-- Extra info for other server people (linked by id)
CREATE TABLE IF NOT EXISTS other_server_people_extra_info (
    other_id INT NOT NULL,
    -- Use id as foreign key
    name VARCHAR(255),
    surname VARCHAR(255),
    age INT,
    location VARCHAR(255),
    occupation VARCHAR(255),
    extra_info TEXT,
    FOREIGN KEY (other_id) REFERENCES other_server_people (id) ON DELETE CASCADE
);
-- Table for storing phone numbers for people on your server
CREATE TABLE IF NOT EXISTS user_server_phone_numbers (
    user_id INT NOT NULL,
    -- Use id as foreign key
    phone_number VARCHAR(20),
    FOREIGN KEY (user_id) REFERENCES user_server_people (id) ON DELETE CASCADE
);
-- Table for storing phone numbers for people on the other server
CREATE TABLE IF NOT EXISTS other_server_phone_numbers (
    other_id INT NOT NULL,
    -- Use id as foreign key
    phone_number VARCHAR(20),
    FOREIGN KEY (other_id) REFERENCES other_server_people (id) ON DELETE CASCADE
);
-- Table for storing social media information for people on your server
CREATE TABLE IF NOT EXISTS user_server_social_media (
    user_id INT NOT NULL,
    -- Use id as foreign key
    facebook VARCHAR(255),
    instagram VARCHAR(255),
    github VARCHAR(255),
    website VARCHAR(255),
    extra_social VARCHAR(255),
    FOREIGN KEY (user_id) REFERENCES user_server_people (id) ON DELETE CASCADE
);
-- Table for storing social media information for people on the other server
CREATE TABLE IF NOT EXISTS other_server_social_media (
    other_id INT NOT NULL,
    -- Use id as foreign key
    facebook VARCHAR(255),
    instagram VARCHAR(255),
    github VARCHAR(255),
    website VARCHAR(255),
    extra_social VARCHAR(255),
    FOREIGN KEY (other_id) REFERENCES other_server_people (id) ON DELETE CASCADE
);
-- Table for storing profile pictures for people on your server
CREATE TABLE IF NOT EXISTS user_server_profile_pictures (
    user_id INT NOT NULL,
    -- Use id as foreign key
    profile_picture_path VARCHAR(255),
    -- Store file path to profile picture
    profile_picture_data LONGBLOB,
    -- Optionally store the image as binary data
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES user_server_people (id) ON DELETE CASCADE
);
-- Table for storing profile pictures for people on the other server
CREATE TABLE IF NOT EXISTS other_server_profile_pictures (
    other_id INT NOT NULL,
    -- Use id as foreign key
    profile_picture_path VARCHAR(255),
    -- Store file path to profile picture
    profile_picture_data LONGBLOB,
    -- Optionally store the image as binary data
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (other_id) REFERENCES other_server_people (id) ON DELETE CASCADE
);
-- 4. Trigger: Create empty records in phone and social media tables when a user is added
CREATE TRIGGER after_user_server_people
AFTER
INSERT ON user_server_people FOR EACH ROW BEGIN -- Create empty record in phone numbers table
INSERT INTO user_server_phone_numbers (user_id, phone_number)
VALUES (NEW.id, '');
-- Create empty record in social media table
INSERT INTO user_server_social_media (
        user_id,
        facebook,
        instagram,
        github,
        website,
        extra_social
    )
VALUES (NEW.id, '', '', '', '', '');
-- Create empty extra info record
INSERT INTO user_server_people_extra_info (
        user_id,
        name,
        surname,
        age,
        location,
        occupation,
        extra_info
    )
VALUES (NEW.id, '', '', NULL, '', '', '');
INSERT INTO user_server_profile_pictures (
        user_id,
        profile_picture_path,
        profile_picture_data
    )
VALUES (NEW.id, '', '');
END;
CREATE TRIGGER after_other_server_people
AFTER
INSERT ON other_server_people FOR EACH ROW BEGIN -- Create empty record in phone numbers table
INSERT INTO other_server_phone_numbers (other_id, phone_number)
VALUES (NEW.id, '');
-- Create empty record in social media table
INSERT INTO other_server_social_media (
        other_id,
        facebook,
        instagram,
        github,
        website,
        extra_social
    )
VALUES (NEW.id, '', '', '', '', '');
-- Create empty extra info record
INSERT INTO other_server_people_extra_info (
        other_id,
        name,
        surname,
        age,
        location,
        occupation,
        extra_info
    )
VALUES (NEW.id, '', '', NULL, '', '', '');
INSERT INTO other_server_profile_pictures (
        other_id,
        profile_picture_path,
        profile_picture_data
    )
VALUES (NEW.id, '', '');
END;
--  messages part: 
CREATE TABLE IF NOT EXISTS messages_send_to_user_client (
    id INT AUTO_INCREMENT PRIMARY KEY,
    sender VARCHAR(255) NOT NULL,
    receiver VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    close_one_point VARCHAR(255),
    connected VARCHAR(255),
    has_attachment BOOLEAN DEFAULT false,
    -- New field for checking if a message has an attachment
    INDEX (connected),
    INDEX (close_one_point)
);
-- Create table for messages sent to other client
CREATE TABLE IF NOT EXISTS messages_send_to_other_client (
    id INT AUTO_INCREMENT PRIMARY KEY,
    sender VARCHAR(255) NOT NULL,
    receiver VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    close_one_point VARCHAR(255),
    connected VARCHAR(255),
    has_attachment BOOLEAN DEFAULT false,
    -- New field for checking if a message has an attachment
    INDEX (connected),
    INDEX (close_one_point)
);
-- Adjust auto-increment for message tables
ALTER TABLE messages_send_to_user_client AUTO_INCREMENT = 1;
ALTER TABLE messages_send_to_other_client AUTO_INCREMENT = 1;
-- Create table for storing sent files information
CREATE TABLE IF NOT EXISTS sent_files (
    id INT AUTO_INCREMENT PRIMARY KEY,
    -- Unique file ID
    file_name VARCHAR(255) NOT NULL,
    -- File name
    file_type VARCHAR(50) NOT NULL,
    -- Type of the file (image, document, gif, etc.)
    file_size BIGINT,
    -- Size of the file in bytes
    file_data LONGBLOB,
    -- File data stored as binary
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- Timestamp of when the file was uploaded
);
-- messages_send_to_user_client_files Table (Linked Files to User Messages)
CREATE TABLE IF NOT EXISTS messages_send_to_user_client_files (
    message_id INT,
    -- Foreign key linking to messages
    file_id INT,
    -- Foreign key linking to sent_files
    PRIMARY KEY (message_id, file_id),
    FOREIGN KEY (message_id) REFERENCES messages_send_to_user_client(id) ON DELETE CASCADE,
    FOREIGN KEY (file_id) REFERENCES sent_files(id) ON DELETE CASCADE
);
-- messages_send_to_other_client_files Table (Linked Files to Other Messages)
CREATE TABLE IF NOT EXISTS messages_send_to_other_client_files (
    message_id INT,
    -- Foreign key linking to messages
    file_id INT,
    -- Foreign key linking to sent_files
    PRIMARY KEY (message_id, file_id),
    FOREIGN KEY (message_id) REFERENCES messages_send_to_other_client(id) ON DELETE CASCADE,
    FOREIGN KEY (file_id) REFERENCES sent_files(id) ON DELETE CASCADE
);
-- sent_images Table (For Images)
CREATE TABLE IF NOT EXISTS sent_images (
    file_id INT PRIMARY KEY,
    -- Use the file ID from the sent_files table
    width INT,
    -- Image width
    height INT,
    -- Image height
    thumbnail LONGBLOB,
    -- Optional thumbnail of the image
    FOREIGN KEY (file_id) REFERENCES sent_files(id) ON DELETE CASCADE
);
-- sent_gifs Table (For GIFs)
CREATE TABLE IF NOT EXISTS sent_gifs (
    file_id INT PRIMARY KEY,
    -- Use the file ID from the sent_files table
    loop_count INT DEFAULT 0,
    -- How many times the GIF loops
    duration INT,
    -- Duration of the GIF in seconds
    FOREIGN KEY (file_id) REFERENCES sent_files(id) ON DELETE CASCADE
);
-- sent_documents Table (For Documents)
CREATE TABLE IF NOT EXISTS sent_documents (
    file_id INT PRIMARY KEY,
    -- Use the file ID from the sent_files table
    page_count INT,
    -- Number of pages (for PDF or Word documents)
    FOREIGN KEY (file_id) REFERENCES sent_files(id) ON DELETE CASCADE
);
-- Step 1: Insert into user_server_connection_keys
INSERT INTO user_server_connection_keys (
        user_secret_key,
        other_side_secret_key,
        is_connection_possible,
        connection_link
    )
VALUES (
        'user_secret_key_1',
        'other_side_secret_key_2',
        true,
        'http://user_link1.com'
    ),
    (
        'user_secret_key_3',
        'other_side_secret_key_4',
        false,
        'http://user_link2.com'
    ),
    (
        'user_secret_key_5',
        'other_side_secret_key_6',
        true,
        'http://user_link3.com'
    );
-- Step 2: Insert into user_server_people linking to user_server_connection_keys
INSERT INTO user_server_people (nick, connection_id)
VALUES (
        'Fahrettin',
        (
            SELECT id
            FROM user_server_connection_keys
            WHERE user_secret_key = 'user_secret_key_1'
        )
    ),
    (
        'Orkun',
        (
            SELECT id
            FROM user_server_connection_keys
            WHERE user_secret_key = 'user_secret_key_3'
        )
    ),
    (
        'Sena',
        (
            SELECT id
            FROM user_server_connection_keys
            WHERE user_secret_key = 'user_secret_key_5'
        )
    );
-- INSERT INTO user_server_people_extra_info
-- Insert into user_server_people_extra_info (linked by nick)
INSERT INTO user_server_people_extra_info (
        user_id,
        name,
        surname,
        age,
        location,
        occupation,
        extra_info
    )
VALUES (
        '1',
        'Fahrettin',
        'Yilmaz',
        25,
        'Istanbul',
        'Software Developer',
        'Loves technology and anime'
    ),
    (
        '2',
        'Orkun',
        'Demir',
        26,
        'Ankara',
        'Architect',
        'Enjoys photography and travel'
    ),
    (
        '3 ',
        'Sena',
        'Aydin',
        24,
        'Izmir',
        'Designer',
        'Creative and artistic spirit'
    );
-- Step 1: Insert into other_server_connection_keys
INSERT INTO other_server_connection_keys (
        user_secret_key,
        other_side_secret_key,
        is_connection_possible,
        connection_link
    )
VALUES (
        'user_secret_key_7',
        'other_side_secret_key_8',
        true,
        'http://other_link1.com'
    ),
    (
        'user_secret_key_9',
        'other_side_secret_key_10',
        false,
        'http://other_link2.com'
    ),
    (
        'user_secret_key_11',
        'other_side_secret_key_12',
        true,
        'http://other_link3.com'
    );
-- Step 2: Insert into other_server_people linking to other_server_connection_keys
INSERT INTO other_server_people (nick, connection_id)
VALUES (
        'Cem',
        (
            SELECT id
            FROM other_server_connection_keys
            WHERE user_secret_key = 'user_secret_key_7'
        )
    ),
    (
        'Ayşe',
        (
            SELECT id
            FROM other_server_connection_keys
            WHERE user_secret_key = 'user_secret_key_9'
        )
    ),
    (
        'Mehmet',
        (
            SELECT id
            FROM other_server_connection_keys
            WHERE user_secret_key = 'user_secret_key_11'
        )
    );
-- Insert into other_server_people_extra_info (linked by nick)
INSERT INTO other_server_people_extra_info (
        other_id,
        name,
        surname,
        age,
        location,
        occupation,
        extra_info
    )
VALUES (
        '1',
        'Cem',
        'Guler',
        30,
        'Bursa',
        'Doctor',
        'Passionate about helping others'
    ),
    (
        '2',
        'Ayşe',
        'Kara',
        28,
        'Antalya',
        'Lawyer',
        'Specialized in family law'
    ),
    (
        '3',
        'Mehmet',
        'Yildiz',
        32,
        'Adana',
        'Engineer',
        'Innovative and tech-savvy'
    );
SELECT *
FROM user_server_people;
SELECT *
FROM other_server_people_extra_info;
SELECT *
FROM messages_send_to_user_client;
SELECT *
FROM messages_send_to_other_client;
SELECT age,
    location,
    occupation,
    extra_info
FROM other_server_people_extra_info;