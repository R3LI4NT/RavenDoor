
const canvas = document.getElementById('particleCanvas');
let ctx = canvas.getContext('2d');
let width = window.innerWidth, height = window.innerHeight;
canvas.width = width;
canvas.height = height;
let particles = [];
const PARTICLE_COUNT = 80;

function resizeCanvas() {
  width = window.innerWidth;
  height = window.innerHeight;
  canvas.width = width;
  canvas.height = height;
}

class Particle {
  constructor() {
    this.x = Math.random() * width;
    this.y = Math.random() * height;
    this.vx = (Math.random() - 0.5) * 0.3;
    this.vy = (Math.random() - 0.5) * 0.2 + 0.2;
    this.size = Math.random() * 2 + 1;
    this.alpha = Math.random() * 0.5 + 0.2;
    this.color = `rgba(183, 0, 0, ${this.alpha})`;
  }
  update() {
    this.x += this.vx;
    this.y += this.vy;
    if (this.y > height) this.y = 0;
    if (this.x > width) this.x = 0;
    if (this.x < 0) this.x = width;
  }
  draw() {
    ctx.fillStyle = this.color;
    ctx.fillRect(this.x, this.y, this.size, this.size);
  }
}

function initParticles() {
  particles = [];
  for (let i = 0; i < PARTICLE_COUNT; i++) particles.push(new Particle());
}

function animateParticles() {
  ctx.clearRect(0, 0, width, height);
  particles.forEach(p => { p.update(); p.draw(); });
  requestAnimationFrame(animateParticles);
}

window.addEventListener('resize', () => {
  resizeCanvas();
  initParticles();
});
resizeCanvas();
initParticles();
animateParticles();

const copyBtnContact = document.getElementById('copyContactBtn');
if(copyBtnContact) {
  copyBtnContact.addEventListener('click', () => {
    navigator.clipboard.writeText('r3li4nt.contact@keemail.me');
    const originalIcon = copyBtnContact.innerHTML;
    copyBtnContact.innerHTML = '<i class="fas fa-check"></i>';
    setTimeout(() => { copyBtnContact.innerHTML = originalIcon; }, 1500);
  });
}

const mobileMenuBtn = document.getElementById('mobileMenuBtn');
const mobileMenu = document.getElementById('mobileMenu');
const mobileMenuClose = document.getElementById('mobileMenuClose');

if(mobileMenuBtn) {
  mobileMenuBtn.addEventListener('click', () => {
    mobileMenu.classList.add('active');
    document.body.style.overflow = 'hidden';
  });
}

if(mobileMenuClose) {
  mobileMenuClose.addEventListener('click', () => {
    mobileMenu.classList.remove('active');
    document.body.style.overflow = '';
  });
}

const mobileLinks = document.querySelectorAll('.mobile-menu a');
mobileLinks.forEach(link => {
  link.addEventListener('click', () => {
    mobileMenu.classList.remove('active');
    document.body.style.overflow = '';
  });
});

document.addEventListener('click', (e) => {
  if(mobileMenu && mobileMenu.classList.contains('active')) {
    if(!mobileMenu.contains(e.target) && !mobileMenuBtn.contains(e.target)) {
      mobileMenu.classList.remove('active');
      document.body.style.overflow = '';
    }
  }
});

const modal = document.getElementById('imageModal');
const modalImg = document.getElementById('modalImage');
const modalClose = document.getElementById('modalClose');

const clickableImages = document.querySelectorAll('.clickable-img');

function openModal(imgSrc) {
  modalImg.src = imgSrc;
  modal.classList.add('active');
  document.body.style.overflow = 'hidden';
}

function closeModal() {
  modal.classList.remove('active');
  document.body.style.overflow = '';
  setTimeout(() => {
    if(!modal.classList.contains('active')) {
      modalImg.src = '';
    }
  }, 300);
}

clickableImages.forEach(img => {
  img.addEventListener('click', (e) => {
    e.stopPropagation();
    openModal(img.src);
  });
});

if(modalClose) {
  modalClose.addEventListener('click', closeModal);
}

if(modal) {
  modal.addEventListener('click', (e) => {
    if(e.target === modal) {
      closeModal();
    }
  });
}

document.addEventListener('keydown', (e) => {
  if(e.key === 'Escape' && modal && modal.classList.contains('active')) {
    closeModal();
  }
});

const badges = document.querySelectorAll('.badge');
badges.forEach(b => {
  b.addEventListener('mouseenter', () => {
    b.style.transform = 'translateY(-2px)';
  });
  b.addEventListener('mouseleave', () => {
    b.style.transform = 'translateY(0)';
  });
});

document.querySelectorAll('a[href^="#"]').forEach(anchor => {
  anchor.addEventListener('click', function(e) {
    e.preventDefault();
    const targetId = this.getAttribute('href');
    if(targetId === '#') return;
    const targetElement = document.querySelector(targetId);
    if(targetElement) {
      targetElement.scrollIntoView({
        behavior: 'smooth',
        block: 'start'
      });
    }
  });
});

const heroImg = document.querySelector('.hero-logo-under img');
if(heroImg) {
  setInterval(() => {
    heroImg.style.transform = 'scale(1.02)';
    setTimeout(() => {
      heroImg.style.transform = 'scale(1)';
    }, 300);
  }, 4000);
}