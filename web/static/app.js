// ReoPal Web Viewer - Frontend Application

class ReoPalApp {
    constructor() {
        this.currentView = 'dashboard';
        this.currentPage = 1;
        this.currentFilters = {};
        this.videos = [];
        this.cameras = [];
        this.totalVideos = 0;

        this.init();
    }

    init() {
        this.bindEvents();
        this.loadDashboard();
    }

    bindEvents() {
        // Navigation
        document.querySelectorAll('.nav-button').forEach(button => {
            button.addEventListener('click', (e) => {
                this.switchView(e.target.dataset.view);
            });
        });

        // Modal
        document.getElementById('modal-close').addEventListener('click', () => {
            this.closeModal();
        });

        // Search
        document.getElementById('search-button').addEventListener('click', () => {
            this.performSearch();
        });

        document.getElementById('search-input').addEventListener('keypress', (e) => {
            if (e.key === 'Enter') {
                this.performSearch();
            }
        });

        // Filters
        document.getElementById('camera-filter').addEventListener('change', () => {
            this.applyFilters();
        });

        document.getElementById('date-from').addEventListener('change', () => {
            this.applyFilters();
        });

        document.getElementById('date-to').addEventListener('change', () => {
            this.applyFilters();
        });

        // Modal background click
        document.getElementById('video-modal').addEventListener('click', (e) => {
            if (e.target.id === 'video-modal') {
                this.closeModal();
            }
        });

        // Refresh button
        document.getElementById('refresh-button').addEventListener('click', () => {
            this.refreshMetadata();
        });

        // Notification close
        document.getElementById('notification-close').addEventListener('click', () => {
            this.hideNotification();
        });
    }

    switchView(view) {
        // Update active navigation button
        document.querySelectorAll('.nav-button').forEach(button => {
            button.classList.remove('active');
        });
        document.querySelector(`[data-view="${view}"]`).classList.add('active');

        // Update active view
        document.querySelectorAll('.view').forEach(viewElement => {
            viewElement.classList.remove('active');
        });
        document.getElementById(`${view}-view`).classList.add('active');

        this.currentView = view;

        // Load view-specific data
        switch (view) {
            case 'dashboard':
                this.loadDashboard();
                break;
            case 'videos':
                this.loadVideos();
                break;
            case 'cameras':
                this.loadCameras();
                break;
            case 'search':
                // Search view is reactive, no initial load needed
                break;
        }
    }

    async loadDashboard() {
        this.showSpinner();
        try {
            const [videosResponse, camerasResponse] = await Promise.all([
                this.apiCall('/api/videos?limit=6'),
                this.apiCall('/api/cameras')
            ]);

            this.updateDashboardStats(videosResponse.total, camerasResponse.length);
            this.renderRecentVideos(videosResponse.videos);
        } catch (error) {
            console.error('Error loading dashboard:', error);
            this.showError('Failed to load dashboard data');
        } finally {
            this.hideSpinner();
        }
    }

    async loadVideos(page = 1) {
        this.showSpinner();
        try {
            const params = new URLSearchParams({
                page: page,
                limit: 20,
                ...this.currentFilters
            });

            const response = await this.apiCall(`/api/videos?${params}`);
            this.videos = response.videos;
            this.totalVideos = response.total;
            this.currentPage = page;

            this.renderVideos(response.videos);
            this.renderPagination(response.total, response.page, response.limit);
        } catch (error) {
            console.error('Error loading videos:', error);
            this.showError('Failed to load videos');
        } finally {
            this.hideSpinner();
        }
    }

    async loadCameras() {
        this.showSpinner();
        try {
            const cameras = await this.apiCall('/api/cameras');
            this.cameras = cameras;
            this.renderCameras(cameras);
            this.populateCameraFilter(cameras);
        } catch (error) {
            console.error('Error loading cameras:', error);
            this.showError('Failed to load cameras');
        } finally {
            this.hideSpinner();
        }
    }

    async performSearch() {
        const query = document.getElementById('search-input').value.trim();
        if (!query) return;

        this.showSpinner();
        try {
            const searchData = { query };
            const results = await this.apiCall('/api/videos/search', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(searchData)
            });

            this.renderSearchResults(results);
        } catch (error) {
            console.error('Error performing search:', error);
            this.showError('Search failed');
        } finally {
            this.hideSpinner();
        }
    }

    applyFilters() {
        this.currentFilters = {
            camera: document.getElementById('camera-filter').value,
            date_from: document.getElementById('date-from').value,
            date_to: document.getElementById('date-to').value
        };

        // Remove empty filters
        Object.keys(this.currentFilters).forEach(key => {
            if (!this.currentFilters[key]) {
                delete this.currentFilters[key];
            }
        });

        this.loadVideos(1);
    }

    updateDashboardStats(totalVideos, totalCameras) {
        document.getElementById('total-videos').textContent = totalVideos;
        document.getElementById('total-cameras').textContent = totalCameras;
        document.getElementById('last-update').textContent = new Date().toLocaleString();

        // Calculate storage (simplified)
        const averageSize = 100; // MB per video estimate
        const totalStorage = (totalVideos * averageSize / 1024).toFixed(1);
        document.getElementById('total-storage').textContent = `${totalStorage} GB`;
    }

    renderRecentVideos(videos) {
        const container = document.getElementById('recent-videos');
        container.innerHTML = videos.map(video => this.createVideoCard(video)).join('');
    }

    renderVideos(videos) {
        const container = document.getElementById('videos-grid');
        container.innerHTML = videos.map(video => this.createVideoCard(video)).join('');
    }

    renderSearchResults(videos) {
        const container = document.getElementById('search-results-grid');
        container.innerHTML = videos.map(video => this.createVideoCard(video)).join('');
    }

    renderCameras(cameras) {
        const container = document.getElementById('cameras-grid');
        container.innerHTML = cameras.map(camera => this.createCameraCard(camera)).join('');
    }

    createVideoCard(video) {
        const startTime = this.formatTime(video.start_time);
        const endTime = this.formatTime(video.end_time);
        const fileSize = this.formatFileSize(video.file_size);
        const date = this.formatDate(video.date);

        return `
            <div class="video-card" onclick="app.playVideo('${video.id}')">
                <div class="video-thumbnail">
                    🎬
                </div>
                <div class="video-info">
                    <div class="video-title">${video.camera_name}</div>
                    <div class="video-meta">
                        <span>📅 ${date}</span>
                        <span>⏰ ${startTime} - ${endTime}</span>
                        <span>📄 ${fileSize}</span>
                        ${video.deleted ? '<span style="color: #f85149;">🗑️ Deleted</span>' : ''}
                    </div>
                </div>
            </div>
        `;
    }

    createCameraCard(camera) {
        const lastRecording = camera.last_recording ? this.formatDateTime(camera.last_recording) : 'Never';

        return `
            <div class="camera-card">
                <div class="camera-name">${camera.name}</div>
                <div class="camera-stats">
                    <span>📹 ${camera.video_count} recordings</span>
                    <span>🕒 Last: ${lastRecording}</span>
                </div>
            </div>
        `;
    }

    renderPagination(total, currentPage, limit) {
        const totalPages = Math.ceil(total / limit);
        const container = document.getElementById('pagination');

        if (totalPages <= 1) {
            container.innerHTML = '';
            return;
        }

        let paginationHTML = '';

        // Previous button
        if (currentPage > 1) {
            paginationHTML += `<button onclick="app.loadVideos(${currentPage - 1})">Previous</button>`;
        }

        // Page numbers
        const startPage = Math.max(1, currentPage - 2);
        const endPage = Math.min(totalPages, currentPage + 2);

        for (let i = startPage; i <= endPage; i++) {
            const activeClass = i === currentPage ? 'active' : '';
            paginationHTML += `<button class="${activeClass}" onclick="app.loadVideos(${i})">${i}</button>`;
        }

        // Next button
        if (currentPage < totalPages) {
            paginationHTML += `<button onclick="app.loadVideos(${currentPage + 1})">Next</button>`;
        }

        container.innerHTML = paginationHTML;
    }

    populateCameraFilter(cameras) {
        const select = document.getElementById('camera-filter');
        select.innerHTML = '<option value="">All Cameras</option>';

        cameras.forEach(camera => {
            select.innerHTML += `<option value="${camera.name}">${camera.name}</option>`;
        });
    }

    async playVideo(videoId) {
        this.showSpinner();
        try {
            const video = await this.apiCall(`/api/videos/${videoId}`);
            this.openVideoModal(video);
        } catch (error) {
            console.error('Error loading video:', error);
            this.showError('Failed to load video');
        } finally {
            this.hideSpinner();
        }
    }

    openVideoModal(video) {
        const modal = document.getElementById('video-modal');
        const player = document.getElementById('video-player');
        const title = document.getElementById('modal-title');
        const videoInfo = document.getElementById('video-info');

        title.textContent = `${video.camera_name} - ${this.formatDate(video.date)}`;

        // Set video source
        player.src = `/api/videos/${video.id}/stream`;

        // Update video info
        videoInfo.innerHTML = `
            <div class="video-meta">
                <p><strong>Camera:</strong> ${video.camera_name}</p>
                <p><strong>Date:</strong> ${this.formatDate(video.date)}</p>
                <p><strong>Time:</strong> ${this.formatTime(video.start_time)} - ${this.formatTime(video.end_time)}</p>
                <p><strong>Size:</strong> ${this.formatFileSize(video.file_size)}</p>
                <p><strong>File:</strong> ${video.file_path}</p>
            </div>
        `;

        modal.classList.add('show');
        document.body.style.overflow = 'hidden';
    }

    closeModal() {
        const modal = document.getElementById('video-modal');
        const player = document.getElementById('video-player');

        modal.classList.remove('show');
        player.pause();
        player.src = '';
        document.body.style.overflow = 'auto';
    }

    async apiCall(endpoint, options = {}) {
        const response = await fetch(endpoint, {
            ...options,
            headers: {
                'Content-Type': 'application/json',
                ...options.headers
            }
        });

        if (!response.ok) {
            throw new Error(`API call failed: ${response.status}`);
        }

        return response.json();
    }

    showSpinner() {
        document.getElementById('spinner').classList.add('show');
    }

    hideSpinner() {
        document.getElementById('spinner').classList.remove('show');
    }

    showError(message) {
        // Simple error display - could be enhanced with a proper notification system
        console.error(message);
        this.showNotification(message, 'error');
    }

    async refreshMetadata() {
        const refreshButton = document.getElementById('refresh-button');
        const refreshIcon = refreshButton.querySelector('.refresh-icon');
        const refreshText = refreshButton.querySelector('.refresh-text');

        // Set loading state
        refreshButton.disabled = true;
        refreshButton.classList.add('loading');
        refreshText.textContent = 'Refreshing...';

        try {
            const response = await this.apiCall('/api/import', {
                method: 'POST'
            });

            if (response.status === 'success') {
                this.showNotification('Video metadata refreshed successfully!', 'success');

                // Refresh current view data
                switch (this.currentView) {
                    case 'dashboard':
                        this.loadDashboard();
                        break;
                    case 'videos':
                        this.loadVideos(this.currentPage);
                        break;
                    case 'cameras':
                        this.loadCameras();
                        break;
                }
            } else {
                this.showNotification(response.message || 'Refresh failed', 'error');
            }
        } catch (error) {
            console.error('Error refreshing metadata:', error);
            this.showNotification('Failed to refresh metadata', 'error');
        } finally {
            // Reset button state
            refreshButton.disabled = false;
            refreshButton.classList.remove('loading');
            refreshText.textContent = 'Refresh';
        }
    }

    showNotification(message, type = 'success') {
        const notification = document.getElementById('notification');
        const icon = document.getElementById('notification-icon');
        const messageEl = document.getElementById('notification-message');

        // Set content
        messageEl.textContent = message;
        icon.textContent = type === 'success' ? '✅' : '❌';

        // Set type
        notification.className = `notification ${type}`;

        // Show notification
        notification.classList.add('show');

        // Auto-hide after 5 seconds
        setTimeout(() => {
            this.hideNotification();
        }, 5000);
    }

    hideNotification() {
        const notification = document.getElementById('notification');
        notification.classList.remove('show');
    }

    // Utility functions
    formatTime(timeStr) {
        if (!timeStr || timeStr.length !== 6) return timeStr;
        return `${timeStr.substring(0, 2)}:${timeStr.substring(2, 4)}:${timeStr.substring(4, 6)}`;
    }

    formatDate(dateStr) {
        if (!dateStr || dateStr.length !== 8) return dateStr;
        const month = dateStr.substring(0, 2);
        const day = dateStr.substring(2, 4);
        const year = dateStr.substring(4, 8);
        return `${month}/${day}/${year}`;
    }

    formatDateTime(dateTimeStr) {
        if (!dateTimeStr || dateTimeStr.length !== 14) return dateTimeStr;
        const date = dateTimeStr.substring(0, 8);
        const time = dateTimeStr.substring(8, 14);
        return `${this.formatDate(date)} ${this.formatTime(time)}`;
    }

    formatFileSize(bytes) {
        if (!bytes) return '0 B';
        const k = 1024;
        const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
    }
}

// Initialize the application
const app = new ReoPalApp();

// Keyboard shortcuts
document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') {
        app.closeModal();
    }

    // Ctrl+R or F5 for refresh (prevent default browser refresh)
    if ((e.ctrlKey && e.key === 'r') || e.key === 'F5') {
        e.preventDefault();
        app.refreshMetadata();
    }
});

// Handle browser back/forward
window.addEventListener('popstate', (e) => {
    // Handle navigation state if needed
});

// Auto-refresh dashboard every 5 minutes
setInterval(() => {
    if (app.currentView === 'dashboard') {
        app.loadDashboard();
    }
}, 5 * 60 * 1000); 