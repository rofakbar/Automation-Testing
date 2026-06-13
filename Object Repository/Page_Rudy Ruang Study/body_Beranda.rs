<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Beranda</name>
   <tag></tag>
   <elementGuidId>9b3dc374-4691-4520-9bb0-089d30926c85</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>body</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>ae90378b-0518-489e-8006-a124403f96df</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>


  
    
    
  

  
    Beranda
    Ruangan
    Riwayat
  

  
    
      Login
    
  



  
    
      Selamat Datang di Rudy
      Ruang Study untuk Semua Mahasiswa!
      
        Atur jadwal belajar dan diskusi dengan mudah melalui Rudy. Temukan ruangan study nyaman,
        praktis, dan siap digunakan kapan pun.
      
      
        Lihat daftar Ruangan
        Lihat Cara Booking
      
    

    
      
    
  

  
    
      
        
        Cepat &amp; Praktis
        Pemesanan ruangan hanya beberapa klik.
      

      
        
        Terintegrasi Perpustakaan
        Data ruangan langsung dari perpustakaan.
      

      
        
        Fleksibel &amp; Transparan
        Atur jadwal sesuai kebutuhanmu.
      

      
        
        Mudah Digunakan
        Antarmuka ramah mahasiswa.
      
    
  

  
    
      
        ×
        Login untuk booking &amp; melihat ruangan
        Masuk terlebih dahulu agar kamu bisa booking &amp; lihat daftar ruangan.
        Masuk sekarang
      
  
  
    
      Ruangan Populer di Rudy
      Ruang study favorit mahasiswa!
    
    
                      
          
          
            Lentera Edukasi
            Kapasitas: 2 - 4 orang 
            Status : Tersedia
            Booking sekarang
          
        
                      
          
          
            Ruang Asa
            Kapasitas: 2 - 3 orang 
            Status : Tersedia
            Booking sekarang
          
        
                      
          
          
            Galeri Literasi
            Kapasitas: 5 - 12 orang 
            Status : Tersedia
            Booking sekarang
          
        
        
    
  

  
    Cara Menggunakan Rudy
    
      Login ke akunmu.
      Pilih ruangan yang ingin kamu pinjam.
      Isi formulir peminjaman.
      Dapatkan kode booking.
      Tunjukkan kode booking ke admin perpustakaan.
      Ruangan siap dipakai.
    
  




    
        
            
                
            
            
                Rudi Ruangan Studi adalah platform peminjaman ruangan perpustakaan yang membantu mahasiswa dan staf mengatur penggunaan ruang belajar dengan mudah dan efisien.
            
        

        
            
                Navigasi
                Beranda
                Ruangan
                Panduan
                    
            
                Kontak
                PerpusPNJ@email.com
                0822123456780
                Kampus PNJ, Depok
            
        
    




  document.querySelector('#lihat-cara-booking').addEventListener('click', function(e){
      e.preventDefault();
      document.querySelector('#cara-booking').scrollIntoView({
          behavior: 'smooth'
      });
  });

  (function () {
    const modal = document.getElementById('login-modal');
    if (!modal) return;
    const openModal = () => modal.classList.add('show');
    const closeModal = () => modal.classList.remove('show');

    document.querySelectorAll('.booking-trigger').forEach(btn => {
      btn.addEventListener('click', e => {
        e.preventDefault();
        openModal();
      });
    });

    modal.addEventListener('click', e => {
      if (e.target.classList.contains('modal') ||
          e.target.classList.contains('modal-backdrop') ||
          e.target.classList.contains('modal-close')) {
        closeModal();
      }
    });
  })();

  document.querySelector('#bantuanpanduan')?.addEventListener('click', function(e){
      e.preventDefault();
      document.querySelector('#cara-booking').scrollIntoView({
          behavior: 'smooth'
      });
  });




</value>
      <webElementGuid>c6391be7-8cd8-4821-b56c-074888c5be58</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>parent</name>
      <type>Main</type>
      <value>md5.v1-41cf27ff4e64b5ea05eec61321a3e506</value>
      <webElementGuid>cedf27c1-ee22-4092-9744-054766cca349</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>ef28ce1c-d83e-4957-84e1-42a2588c3d74</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>4e34033a-fabb-4154-8d2d-d59e7ce04722</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;


  
    
    
  

  
    Beranda
    Ruangan
    Riwayat
  

  
    
      Login
    
  



  
    
      Selamat Datang di Rudy
      Ruang Study untuk Semua Mahasiswa!
      
        Atur jadwal belajar dan diskusi dengan mudah melalui Rudy. Temukan ruangan study nyaman,
        praktis, dan siap digunakan kapan pun.
      
      
        Lihat daftar Ruangan
        Lihat Cara Booking
      
    

    
      
    
  

  
    
      
        
        Cepat &amp; Praktis
        Pemesanan ruangan hanya beberapa klik.
      

      
        
        Terintegrasi Perpustakaan
        Data ruangan langsung dari perpustakaan.
      

      
        
        Fleksibel &amp; Transparan
        Atur jadwal sesuai kebutuhanmu.
      

      
        
        Mudah Digunakan
        Antarmuka ramah mahasiswa.
      
    
  

  
    
      
        ×
        Login untuk booking &amp; melihat ruangan
        Masuk terlebih dahulu agar kamu bisa booking &amp; lihat daftar ruangan.
        Masuk sekarang
      
  
  
    
      Ruangan Populer di Rudy
      Ruang study favorit mahasiswa!
    
    
                      
          
          
            Lentera Edukasi
            Kapasitas: 2 - 4 orang 
            Status : Tersedia
            Booking sekarang
          
        
                      
          
          
            Ruang Asa
            Kapasitas: 2 - 3 orang 
            Status : Tersedia
            Booking sekarang
          
        
                      
          
          
            Galeri Literasi
            Kapasitas: 5 - 12 orang 
            Status : Tersedia
            Booking sekarang
          
        
        
    
  

  
    Cara Menggunakan Rudy
    
      Login ke akunmu.
      Pilih ruangan yang ingin kamu pinjam.
      Isi formulir peminjaman.
      Dapatkan kode booking.
      Tunjukkan kode booking ke admin perpustakaan.
      Ruangan siap dipakai.
    
  




    
        
            
                
            
            
                Rudi Ruangan Studi adalah platform peminjaman ruangan perpustakaan yang membantu mahasiswa dan staf mengatur penggunaan ruang belajar dengan mudah dan efisien.
            
        

        
            
                Navigasi
                Beranda
                Ruangan
                Panduan
                    
            
                Kontak
                PerpusPNJ@email.com
                0822123456780
                Kampus PNJ, Depok
            
        
    




  document.querySelector(&quot; , &quot;'&quot; , &quot;#lihat-cara-booking&quot; , &quot;'&quot; , &quot;).addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e){
      e.preventDefault();
      document.querySelector(&quot; , &quot;'&quot; , &quot;#cara-booking&quot; , &quot;'&quot; , &quot;).scrollIntoView({
          behavior: &quot; , &quot;'&quot; , &quot;smooth&quot; , &quot;'&quot; , &quot;
      });
  });

  (function () {
    const modal = document.getElementById(&quot; , &quot;'&quot; , &quot;login-modal&quot; , &quot;'&quot; , &quot;);
    if (!modal) return;
    const openModal = () => modal.classList.add(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
    const closeModal = () => modal.classList.remove(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);

    document.querySelectorAll(&quot; , &quot;'&quot; , &quot;.booking-trigger&quot; , &quot;'&quot; , &quot;).forEach(btn => {
      btn.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, e => {
        e.preventDefault();
        openModal();
      });
    });

    modal.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, e => {
      if (e.target.classList.contains(&quot; , &quot;'&quot; , &quot;modal&quot; , &quot;'&quot; , &quot;) ||
          e.target.classList.contains(&quot; , &quot;'&quot; , &quot;modal-backdrop&quot; , &quot;'&quot; , &quot;) ||
          e.target.classList.contains(&quot; , &quot;'&quot; , &quot;modal-close&quot; , &quot;'&quot; , &quot;)) {
        closeModal();
      }
    });
  })();

  document.querySelector(&quot; , &quot;'&quot; , &quot;#bantuanpanduan&quot; , &quot;'&quot; , &quot;)?.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e){
      e.preventDefault();
      document.querySelector(&quot; , &quot;'&quot; , &quot;#cara-booking&quot; , &quot;'&quot; , &quot;).scrollIntoView({
          behavior: &quot; , &quot;'&quot; , &quot;smooth&quot; , &quot;'&quot; , &quot;
      });
  });




&quot;) or . = concat(&quot;


  
    
    
  

  
    Beranda
    Ruangan
    Riwayat
  

  
    
      Login
    
  



  
    
      Selamat Datang di Rudy
      Ruang Study untuk Semua Mahasiswa!
      
        Atur jadwal belajar dan diskusi dengan mudah melalui Rudy. Temukan ruangan study nyaman,
        praktis, dan siap digunakan kapan pun.
      
      
        Lihat daftar Ruangan
        Lihat Cara Booking
      
    

    
      
    
  

  
    
      
        
        Cepat &amp; Praktis
        Pemesanan ruangan hanya beberapa klik.
      

      
        
        Terintegrasi Perpustakaan
        Data ruangan langsung dari perpustakaan.
      

      
        
        Fleksibel &amp; Transparan
        Atur jadwal sesuai kebutuhanmu.
      

      
        
        Mudah Digunakan
        Antarmuka ramah mahasiswa.
      
    
  

  
    
      
        ×
        Login untuk booking &amp; melihat ruangan
        Masuk terlebih dahulu agar kamu bisa booking &amp; lihat daftar ruangan.
        Masuk sekarang
      
  
  
    
      Ruangan Populer di Rudy
      Ruang study favorit mahasiswa!
    
    
                      
          
          
            Lentera Edukasi
            Kapasitas: 2 - 4 orang 
            Status : Tersedia
            Booking sekarang
          
        
                      
          
          
            Ruang Asa
            Kapasitas: 2 - 3 orang 
            Status : Tersedia
            Booking sekarang
          
        
                      
          
          
            Galeri Literasi
            Kapasitas: 5 - 12 orang 
            Status : Tersedia
            Booking sekarang
          
        
        
    
  

  
    Cara Menggunakan Rudy
    
      Login ke akunmu.
      Pilih ruangan yang ingin kamu pinjam.
      Isi formulir peminjaman.
      Dapatkan kode booking.
      Tunjukkan kode booking ke admin perpustakaan.
      Ruangan siap dipakai.
    
  




    
        
            
                
            
            
                Rudi Ruangan Studi adalah platform peminjaman ruangan perpustakaan yang membantu mahasiswa dan staf mengatur penggunaan ruang belajar dengan mudah dan efisien.
            
        

        
            
                Navigasi
                Beranda
                Ruangan
                Panduan
                    
            
                Kontak
                PerpusPNJ@email.com
                0822123456780
                Kampus PNJ, Depok
            
        
    




  document.querySelector(&quot; , &quot;'&quot; , &quot;#lihat-cara-booking&quot; , &quot;'&quot; , &quot;).addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e){
      e.preventDefault();
      document.querySelector(&quot; , &quot;'&quot; , &quot;#cara-booking&quot; , &quot;'&quot; , &quot;).scrollIntoView({
          behavior: &quot; , &quot;'&quot; , &quot;smooth&quot; , &quot;'&quot; , &quot;
      });
  });

  (function () {
    const modal = document.getElementById(&quot; , &quot;'&quot; , &quot;login-modal&quot; , &quot;'&quot; , &quot;);
    if (!modal) return;
    const openModal = () => modal.classList.add(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
    const closeModal = () => modal.classList.remove(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);

    document.querySelectorAll(&quot; , &quot;'&quot; , &quot;.booking-trigger&quot; , &quot;'&quot; , &quot;).forEach(btn => {
      btn.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, e => {
        e.preventDefault();
        openModal();
      });
    });

    modal.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, e => {
      if (e.target.classList.contains(&quot; , &quot;'&quot; , &quot;modal&quot; , &quot;'&quot; , &quot;) ||
          e.target.classList.contains(&quot; , &quot;'&quot; , &quot;modal-backdrop&quot; , &quot;'&quot; , &quot;) ||
          e.target.classList.contains(&quot; , &quot;'&quot; , &quot;modal-close&quot; , &quot;'&quot; , &quot;)) {
        closeModal();
      }
    });
  })();

  document.querySelector(&quot; , &quot;'&quot; , &quot;#bantuanpanduan&quot; , &quot;'&quot; , &quot;)?.addEventListener(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e){
      e.preventDefault();
      document.querySelector(&quot; , &quot;'&quot; , &quot;#cara-booking&quot; , &quot;'&quot; , &quot;).scrollIntoView({
          behavior: &quot; , &quot;'&quot; , &quot;smooth&quot; , &quot;'&quot; , &quot;
      });
  });




&quot;))]</value>
      <webElementGuid>3ad32e1d-a4be-446f-b3d3-0ba33765fb66</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
