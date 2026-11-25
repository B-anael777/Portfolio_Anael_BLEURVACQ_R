pub const INDEX_HTML: &str = r##"<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Portfolio Anael BLEURVACQ</title>
    <style>
        html { scroll-behavior: smooth; }
        body {
            font-family: Arial, sans-serif;
            margin: 0;
            padding-top: 65px;
            line-height: 1.6;
            position: relative;
            overflow-x: hidden;
        }
        #background {
            position: fixed;
            inset: 0;
            width: 100%;
            height: 100%;
            object-fit: cover;
            z-index: -1;
        }
        nav.violet-gradient {
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 65px;
            display: flex;
            justify-content: center;
            align-items: center;
            z-index: 1000;
            background: linear-gradient(to right, #c852ff, #0057ff);
        }
        nav a {
            color: white;
            margin: 0 18px;
            text-decoration: none;
            font-weight: bold;
        }
        nav a:hover { text-decoration: underline; }
        header.violet-gradient,
        footer.violet-gradient {
            text-align: center;
            background: linear-gradient(to right, #c852ff, #0057ff);
            color: white;
        }
        header.violet-gradient { padding: 20px 0; margin: 0; }
        footer.violet-gradient { padding: 20px 0; margin-top: 20px; }
        .photo-profil img {
            width: 150px;
            height: 150px;
            object-fit: cover;
            border-radius: 50%;
            border: 4px solid white;
        }
        h1, h2, h3, h4 { margin: 10px 0; }
        main {
            width: 90%;
            max-width: 1000px;
            margin: 30px auto;
        }
        section {
            margin-bottom: 40px;
            scroll-margin-top: 80px;
            padding: 20px;
            border-radius: 10px;
            background: rgba(255, 255, 255, 0.7);
            backdrop-filter: blur(10px);
            box-shadow: 0px 0px 10px rgba(0,0,0,0.1);
        }
        .btn {
            display: inline-block;
            padding: 10px 20px;
            margin-top: 10px;
            border-radius: 5px;
            text-decoration: none;
            color: white;
            transition: background 0.3s;
        }
        .btn.violet-gradient {
            background: linear-gradient(to right, #c852ff, #0057ff);
        }
        .btn.violet-gradient:hover {
            background: linear-gradient(to right, #0057ff, #c852ff);
        }
        #projets {
            position: relative;
            overflow: hidden;
            background: rgba(6, 22, 34, 0.72);
            border: 1px solid rgba(120, 220, 255, 0.8);
            box-shadow: 0 0 22px rgba(38, 173, 255, 0.2), inset 0 0 18px rgba(97, 191, 255, 0.14);
            color: #edfaff;
            backdrop-filter: blur(10px);
            -webkit-backdrop-filter: blur(10px);
        }
        #projets::before {
            content: "";
            position: absolute;
            inset: 0;
            background:
                linear-gradient(135deg, rgba(76, 215, 255, 0.12), rgba(16, 82, 152, 0.04)),
                repeating-linear-gradient(90deg, rgba(140, 224, 255, 0.22) 0 2px, transparent 2px 90px),
                repeating-linear-gradient(0deg, rgba(140, 224, 255, 0.22) 0 2px, transparent 2px 90px);
            opacity: 0.9;
            pointer-events: none;
        }
        #projets > * { position: relative; z-index: 1; }
        #projets .projet {
            background: rgba(12, 31, 44, 0.55);
            border: 1px solid rgba(120, 220, 255, 0.7);
            border-radius: 14px;
            padding: 18px;
            margin-top: 18px;
            box-shadow: inset 0 0 12px rgba(100, 201, 255, 0.12);
        }
        .procedure-buttons a { display: block; margin: 5px 0; }
        .placeholder { color: #555; font-style: italic; }
        #projets .placeholder { color: #dff8ff; }
        video {
            width: 100%;
            max-width: 300px;
            display: block;
            margin: 20px auto;
        }
        .ecole-link {
            display: flex;
            align-items: center;
            gap: 10px;
            justify-content: center;
            text-decoration: none;
            color: inherit;
        }
        .ecole-link img { width: 200px; height: auto; }
        @media (max-width: 768px) {
            body { padding-top: 120px; }
            nav.violet-gradient {
                height: auto;
                min-height: 65px;
                padding: 10px 12px;
                flex-wrap: wrap;
                gap: 8px 12px;
            }
            nav a { margin: 0; font-size: 0.75rem; line-height: 1.2; }
            header.violet-gradient { padding: 18px 12px; }
            h1 { font-size: 1.6rem; }
            h2 { font-size: 1.4rem; }
            h3 { font-size: 1.1rem; line-height: 1.4; }
            main { width: 95%; }
            section { padding: 14px; }
            .photo-profil img { width: 110px; height: 110px; }
            .btn, .procedure-buttons a {
                display: block;
                width: 100%;
                max-width: 100%;
                box-sizing: border-box;
                text-align: center;
            }
            .procedure-buttons a { margin: 8px 0; }
            #projets .projet img, #projets .projet video,
            .ecole-link img, video, img {
                width: 100%;
                max-width: 100%;
                height: auto;
            }
            .ecole-link { flex-direction: column; gap: 8px; }
            .ecole-link img { width: 140px; }
        }
    </style>
</head>
<body>
    <img id="background" src="photo_de_fond.jpg" alt="Fond du portfolio">

    <nav class="violet-gradient">
        <a href="#cv">CV</a>
        <a href="#rapport-stage">Stage 2026</a>
        <a href="#resume-bts">Résumé BTS SIO SLAM</a>
        <a href="#projets">Projets informatiques</a>
        <a href="#ecole">Site de l'école</a>
        <a href="#procedures">Procédures informatiques</a>
        <a href="#contact">Contact</a>
    </nav>

    <header class="violet-gradient">
        <div class="photo-profil">
            <img src="photo.png.png" alt="Photo de Anael BLEURVACQ">
        </div>
        <h1>Mon Portfolio - Anael BLEURVACQ</h1>
    </header>

    <main>
        <section id="parcours-professionnel">
            <h2>Parcours professionnel</h2>
            <div class="experience">
                <h3>Stage de 3e - Décembre 2021 <span style="display:inline-block; background: linear-gradient(to right, #c852ff, #0057ff); -webkit-background-clip: text; background-clip: text; color: transparent; font-weight: bold;">(5j)</span></h3>
                <p>CNRS - Laboratoire APC - Paris</p>
            </div>
            <div class="experience">
                <h3>Stage de 1ere - Octobre 2021 <span style="display:inline-block; background: linear-gradient(to right, #c852ff, #0057ff); -webkit-background-clip: text; background-clip: text; color: transparent; font-weight: bold;">(9j)</span></h3>
                <p>Société Générale - La Défense</p>
            </div>
            <div class="experience">
                <h3>Bac STI2D SIN <span style="display:inline-block; background: linear-gradient(to right, #c852ff, #0057ff); -webkit-background-clip: text; background-clip: text; color: transparent; font-weight: bold;">(2023-2025)</span></h3>
                <p>Notre Dame De Sion - Evry-Courcouronnes<br>Site du collège / lycée privée :</p>
                <a href="https://sion91.fr/" target="_blank" class="ecole-link">
                    <img src="Logo.NDS.jpg" alt="Logo école">
                </a>
            </div>
            <div class="experience">
                <h3>BTS SIO SLAM en alternance <span style="display:inline-block; background: linear-gradient(to right, #c852ff, #0057ff); -webkit-background-clip: text; background-clip: text; color: transparent; font-weight: bold;">(Septembre 2025 à novembre 2025)</span></h3>
            </div>
            <div class="experience">
                <h3>BTS SIO SLAM en initial de Décembre 2025 à Mai 2027 <span style="display:inline-block; background: linear-gradient(to right, #c852ff, #0057ff); -webkit-background-clip: text; background-clip: text; color: transparent; font-weight: bold;">(En cours)</span></h3>
                <a href="#ecole" class="btn violet-gradient">Site du campus</a>
            </div>
        </section>

        <section id="competences-techniques">
            <h2>Compétences techniques</h2>
            <p class="placeholder">
                HTML, CSS, SQL (bases de données), C++, C#, Python<br>
                SolidWorks - SketchUp<br>
                Suite Office<br>
                Hardware (montage PC, dépannage)<br>
                Réseaux (configuration routeurs, cablâge, Wi-Fi)<br>
                Software en général
            </p>
        </section>

        <section id="cv">
            <h2>Mon CV</h2>
            <a href="CV-BLEURVACQ_Anaël.pdf" class="btn violet-gradient" download>Télécharger CV</a>
        </section>

        <section id="rapport-stage">
            <h2>Mon rapport de stage BTS SIO SLAM première année (18 mai au 26 juin 2026)</h2>
            <p>Modification du site officiel GESTIUT de l'université de Ville d'Avray : partie élève et prof<br>(exemple : Pronote, École direct, ENT, netypareo)</p>
            <a href="BLEURVACQ-Anael-SIO1-rapport de stage.pdf" class="btn violet-gradient" download>Télécharger le rapport</a>
        </section>

        <section id="resume-bts">
            <h2>Résumé BTS SIO SLAM</h2>
            <a href="https://institutionsaintaspais.fr/formations/bts-services-informatiques-aux-organisations-option-slam-et-sisr/" target="_blank" class="btn violet-gradient">
                Voir le site du BTS SIO SLAM
            </a>
            <p>Le BTS SIO (Services Informatiques aux Organisations) est une formation de 2 ans après le bac qui prépare aux métiers de l’informatique en entreprise.</p>
            <p>Elle forme à l’administration des systèmes et réseaux ou au développement d’applications, selon l’option choisie (SISR ou SLAM).</p>
            <p>Les étudiants y apprennent la programmation, les bases de données, la sécurité informatique, la gestion de projets et le support aux utilisateurs.</p>
            <p>La formation est très professionnalisante, avec des stages en entreprise et permet soit une insertion rapide dans le monde du travail, soit une poursuite d’études.</p>
        </section>

        <section id="projets">
            <h2>Projets informatiques réalisés</h2>
            <div class="projet">
                <h3>Formation dispensée par Apple en 2019</h3>
                <p class="placeholder">Programmer un robot nommé SPHERO BOLT avec une appli/logiciel sphero edu de type scratch pour lui faire faire des actions.</p>
                <p class="placeholder">( Déplacement, rotation, lumière, son )</p>
                <img src="sphero_bolt.jpeg" alt="SPHERO BOLT" style="width: 220px; height: auto; display: block; margin: 0 auto;">
                <img src="Sph.png" alt="SPHERO BOLT" style="width: 500px; height: auto; display: block; margin: 0 auto;">
            </div>
            <div class="projet">
                <h3>Robot suiveur de ligne en Arduino (Première STI2D)</h3>
                <p class="placeholder">Robot suiveur de ligne programmé en Arduino python dans le cadre d'un projet de classe.<br>
                    Groupe de 2 comparé au reste de la classe de groupes de 3 et 4, premier groupe à avoir réussi<br>
                    Note de fin de projet et d'oral : 17/20<br><br>
                    Exemple de vidéo non fini du robot en action :
                </p>
                <video controls>
                    <source src="1000046943.mp4" type="video/mp4">
                </video>
            </div>
            <div class="projet">
                <h3>Site internet avec caméra connectée (Terminale STI2D)</h3>
                <p class="placeholder">Site web permettant de surveiller un aquarium avec caméra et contrôle de la température.</p>
                <p class="placeholder">De ce style :</p>
                <img src="aquarium.jpeg" alt="Aquarium" style="width: 220px; height: auto; display: block; margin: 0 auto;">
            </div>
            <div class="projet">
                <h3>Stage de première année BTS SIO SLAM (18 mai au 26 juin 2026)</h3>
                <p class="placeholder">Modification du site officiel GESTIUT de l'université de Ville d'Avray : partie élève et prof<br>
                    Modification du CSS et HTML de la partie élève pour améliorer l'expérience utilisateur et la navigation.<br>
                    Modification du PHP et Template (smarty) de la partie prof pour ajouter de nouvelles fonctionnalités de stockage et matériel pour la tracabilité des objets.
                </p>
                <a href="#rapport-stage" class="btn violet-gradient">Voir le rapport de stage</a>
            </div>
        </section>

        <section id="ecole">
            <h2>Site du lycée / campus</h2>
            <a href="https://institutionsaintaspais.fr/" target="_blank" class="ecole-link">
                <img src="logo_SA.png" alt="Logo école">
            </a>
        </section>

        <section id="procedures">
            <h2>Procédures informatiques</h2>
            <div class="procedure-buttons">
                <a href="Procédure 1.docx" class="btn violet-gradient" download>Procédure 1</a>
                <a href="Procédure 2.docx" class="btn violet-gradient" download>Procédure 2</a>
                <a href="Procédure 3.docx" class="btn violet-gradient" download>Procédure 3</a>
            </div>
        </section>

        <section id="mon-url">
            <h2>Mon URL</h2>
            <div class="procedure-buttons">
                <a href="https://b-anael777.github.io/Portfolio_Anael_BLEURVACQ/" target="_blank" class="btn violet-gradient">Voir mon portfolio</a>
            </div>
        </section>

        <section id="contact">
            <h2>Contact</h2>
            <p>Mail : <img src="email.png" alt="Adresse électronique" style="width: 200px;"></p>
            <p>Téléphone : 07 61 88 97 21</p>
            <p>LinkedIn : <a href="https://www.linkedin.com/in/ana%C3%ABl-b-2777a72a1/" target="_blank">Voir mon profil</a> ou <a href="https://www.linkedin.com/in/anael-bleurvacq/" target="_blank">Voir mon profil</a></p>
        </section>
    </main>

    <footer class="violet-gradient">
        &copy; 2026 Anael BLEURVACQ | Tous droits pas réservés
    </footer>
</body>
</html>
"##;
