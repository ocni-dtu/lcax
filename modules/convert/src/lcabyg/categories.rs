pub fn category_id_to_names(category_id: &str) -> &str {
    return match category_id {
        "a48092d1-8b5c-4fa9-9ada-69cec3fd1dca" => "Altanbund",
        "069983d0-d08b-405b-b816-d28ca9648956" => "Andet",
        "97145af4-423d-447f-a76e-b26feba645cd" => "Andet (El- og mekaniske anlæg)",
        "041605d6-5971-4ef5-8861-a8bb299082a7" => "Andet (søjler og bjælker)",
        "4b058227-654f-426f-96ad-44caaf9cdf1b" => "Andet (udendørs areal)",
        "142f0239-b1df-4295-9c52-2f8a8274a211" => "Andet (varme)",
        "0bea536d-2284-4728-80c8-2e01119c4cc7" => "Beklædning",
        "8cbf3a89-57c8-4725-b3c3-481c1d6ce19d" => "Belysning",
        "14bdd9bc-fc11-4846-b89d-5434dd04b773" => "Belægninger",
        "ec9ee040-9c1d-4cae-864c-4c6a0e4b8c5b" => "Bjælker",
        "59ab59a5-2482-45ae-85f1-d0e39e640712" => "Bærende indervægge",
        "2f6f494e-5e60-450d-aa7e-4f5845af39dc" => "Bærende indervægge i kælderen",
        "9bd14e05-5fba-4ecb-ab41-7940a8a0dbdc" => "Døre",
        "205798f2-16d0-4719-89fc-c6d6cf274375" => "El-anlæg",
        "dc4a81ce-2d10-4fba-b282-a3cce4e9a187" => "Energiproduktion",
        "f4c234ec-77f1-4ee0-92d0-f1819e0307d4" => "Etagedæk",
        "4f07b17d-254b-4914-bac6-871c424016b8" => "Faldstammer",
        "98b7d6c8-6ece-4597-af57-faec8d963ca8" => "Fastgørelse",
        "caa0d6a5-228f-44cb-9766-04056b28a825" => "Glasfacader",
        "bf2534e2-87e6-40cb-9fff-17fb3d85a52a" => "Gulv",
        "57c1d3ac-98e7-42ac-b4cc-63c287703bae" => "Gulv",
        "78d66965-b806-47d6-af5f-b278e737adaa" => "Gulvvarme",
        "a9ab6709-61e1-46c1-a2e8-df00bdd0bb91" => "Ikke-bærende indervægge",
        "d401d419-f4f6-45b9-bb18-31658a9a223b" => "Ikke-bærende indervægge i kælderen",
        "02bccdda-7818-4c24-b028-5bfebaff4413" => "Installationer over jord",
        "d4beed46-bf57-49c7-a57f-a9bdf42484a5" => "Installationer under jord",
        "f49cfb86-3081-48da-8a92-cba5022de4c1" => "Kælderdæk",
        "4421b3e3-9b5d-49e9-a654-253c5974db75" => "Kælderydervægge",
        "a77a1779-8794-48bf-9e80-2c0a8bb36596" => "Loft",
        "08c35ced-ebba-4c83-8e49-b9f84e0da056" => "Loft",
        "5fedd254-0ea3-46fc-99db-89d791b7ead1" => "Nedløb fra tag",
        "e582a92e-71d6-4137-89ab-2870dec4a475" => "Pladefundament",
        "e5f1c0f4-431e-4722-919f-35b15793f061" => "Punktfundament",
        "270e865a-07e6-4dbd-b4ff-a5e90edeba60" => "Pælefundering",
        "aea37dbf-bc04-404c-af3e-7f12d7f626b2" => "Radiatorer",
        "1a966c7c-fd7c-4abc-9d4b-307fdcd0cc88" => "Randfundamenter",
        "bdbec7c1-4744-43f8-9d06-84cf75288110" => "Rækværk",
        "670dd86e-1a08-4ef9-900e-1953dd4c1c3c" => "Sekundær bebyggelse",
        "3680fa33-81cb-4681-96cc-86948ad19ba0" => "Spuns",
        "846a848b-fa1a-43c8-85ed-8f2294c06ca5" => "Stribefundamenter under bærende indervægge",
        "506a420e-c3cd-4849-a759-3117f10937b1" => "Søjler",
        "d734712a-d27d-42c5-936f-98fe4c4df90b" => "Tage",
        "0cec4daa-43c6-402a-89ef-4bd3a4b25312" => "Terrænbefæstninger",
        "2ffe16fd-f0c9-4d31-a31a-f96d58d3df95" => "Terrændæk",
        "858395b4-2734-485e-ab34-c3129c7d24e7" => "Transportanlæg",
        "1caeddce-29e9-4067-b85b-36e8d103a4a0" => "Trapper og ramper",
        "c995ac63-9c84-4f58-82d5-5fe7ffd3da89" => "Udendørs belysning",
        "87252bfb-b1f4-460f-b2c3-82cac8f79450" => "Udendørs inventar",
        "801b6a1a-982a-4e64-b6ac-faee4055a1d0" => "Udendørs trapper og ramper",
        "16528722-ecc6-4a69-a269-35ba3e1205b2" => "Vandrør",
        "0fb0348e-2c54-40cb-a4c6-a9c1f2033b22" => "Varmeforsyningsanlæg",
        "895bc4b9-ac42-46c2-8b6b-52a031660614" => "Varmerør",
        "1f691457-9bc8-423c-a107-c0254aa15768" => "Varmtvandsbeholder",
        "cf702d99-5d5a-4973-ba9c-c4dbfae656a9" => "Ventilationsanlæg",
        "58015317-c3ee-41d3-a9df-5fbea8fb2dd2" => "Ventilationskanaler",
        "dcbf55db-3189-4d22-9b2b-2e27c057b363" => "Vinduer",
        "10a52123-48d7-466a-9622-d463511a6df0" => "Ydervægge",
        _ => "Andet",
    };
}