# ReoPal Web Viewer - Product Requirements Document

## Executive Summary

This PRD outlines the development of a web-based application that extends the existing ReoPal CLI tool to provide users with a browser-based interface for viewing, searching, and managing ReoLink security camera footage. The web viewer will leverage the existing SQLite database and video file management infrastructure while providing an intuitive, responsive user interface.

## Product Overview

### Vision Statement
Transform ReoPal from a CLI-only tool into a complete video management solution by adding a web-based viewing interface that makes security camera footage accessible to all users, regardless of technical expertise.

### Problem Statement
- **Current State**: ReoPal efficiently indexes and manages video files via CLI, but users cannot easily browse, search, or view their camera footage
- **Pain Points**: 
  - No visual interface to browse video catalog
  - Difficult to locate specific recordings by time/camera
  - No preview capabilities for video content
  - Limited accessibility for non-technical users
  - No remote access to footage

### Solution Overview
A web-based application that provides:
- Visual browsing of video catalog with thumbnail previews
- Advanced search and filtering capabilities
- In-browser video playback with timeline controls
- Responsive design for desktop and mobile access
- Real-time updates when new footage is indexed

## Target Users

### Primary Users
- **Security Personnel**: Need quick access to specific time periods and cameras
- **Property Owners**: Want to review footage without technical complexity
- **System Administrators**: Require oversight of storage and system health

### Secondary Users
- **Remote Users**: Need access to footage from different locations
- **Mobile Users**: Require mobile-friendly interface for on-the-go access

## Functional Requirements

### Core Features

#### 1. Video Catalog Browser
- **FR-1.1**: Display video recordings in a grid/list view with thumbnails
- **FR-1.2**: Show metadata for each recording (camera name, date, time range, file size)
- **FR-1.3**: Sort recordings by date, camera, duration, or file size
- **FR-1.4**: Paginated results with configurable items per page
- **FR-1.5**: Visual indicators for deleted/archived files

#### 2. Search and Filtering
- **FR-2.1**: Filter by camera name from dropdown/multi-select
- **FR-2.2**: Date range picker for filtering recordings
- **FR-2.3**: Time range filtering (e.g., "only show recordings from 8 AM - 6 PM")
- **FR-2.4**: Quick filters for common time periods (Today, Yesterday, Last 7 Days, etc.)
- **FR-2.5**: Search by filename or camera name
- **FR-2.6**: Advanced filters for file size ranges

#### 3. Video Player
- **FR-3.1**: Embedded HTML5 video player with standard controls
- **FR-3.2**: Playback speed controls (0.5x, 1x, 1.5x, 2x)
- **FR-3.3**: Frame-by-frame navigation capabilities
- **FR-3.4**: Fullscreen mode
- **FR-3.5**: Keyboard shortcuts for common operations
- **FR-3.6**: Video quality selection (if multiple formats available)

#### 4. Timeline and Navigation
- **FR-4.1**: Visual timeline showing all recordings for a selected date
- **FR-4.2**: Gap visualization between recordings
- **FR-4.3**: Quick jump to specific time periods
- **FR-4.4**: Continuous playback across multiple files
- **FR-4.5**: Thumbnail scrubbing for quick preview

#### 5. Camera Management
- **FR-5.1**: Camera overview dashboard with last recording times
- **FR-5.2**: Camera-specific views and filtering
- **FR-5.3**: Camera status indicators (active, inactive, deleted files)
- **FR-5.4**: Bulk operations per camera

#### 6. System Integration
- **FR-6.1**: Real-time updates when CLI import runs
- **FR-6.2**: Integration with existing SQLite database
- **FR-6.3**: Access to all indexed video files
- **FR-6.4**: Respect existing file deletion/archival states

### Advanced Features (Future Considerations)

#### 7. Export and Sharing
- **FR-7.1**: Export video clips for specific time ranges
- **FR-7.2**: Generate shareable links for specific recordings
- **FR-7.3**: Bulk download capabilities
- **FR-7.4**: Video format conversion options

#### 8. Analytics and Reporting
- **FR-8.1**: Storage usage visualization
- **FR-8.2**: Recording frequency charts by camera
- **FR-8.3**: System health monitoring
- **FR-8.4**: Automated report generation

## Technical Requirements

### Architecture

#### Backend Requirements
- **TR-1.1**: RESTful API built with Rust web framework (Axum/Actix-web)
- **TR-1.2**: WebSocket support for real-time updates
- **TR-1.3**: Integration with existing SQLite database
- **TR-1.4**: Video file streaming capabilities
- **TR-1.5**: Thumbnail generation and caching
- **TR-1.6**: Authentication and authorization system

#### Frontend Requirements
- **TR-2.1**: Modern web framework (React/Vue.js/Svelte)
- **TR-2.2**: Responsive design supporting mobile devices
- **TR-2.3**: Progressive Web App (PWA) capabilities
- **TR-2.4**: Client-side caching for improved performance
- **TR-2.5**: Accessibility compliance (WCAG 2.1)

#### Performance Requirements
- **TR-3.1**: Page load time < 3 seconds
- **TR-3.2**: Video playback start time < 2 seconds
- **TR-3.3**: Support for 100+ concurrent users
- **TR-3.4**: Efficient handling of large video files (>1GB)
- **TR-3.5**: Thumbnail generation < 5 seconds per video

#### Security Requirements
- **TR-4.1**: HTTPS enforcement
- **TR-4.2**: Authentication required for access
- **TR-4.3**: Session management with configurable timeout
- **TR-4.4**: Rate limiting for API endpoints
- **TR-4.5**: Input validation and sanitization
- **TR-4.6**: Secure video streaming without exposing file paths

### Configuration Integration

#### Extended Configuration Schema
```yaml
# config.yml - Extended for web viewer
directory: "/mnt/reolink/videos"
db_path: "reopal.db"

# Web viewer configuration
web_viewer:
  # Server configuration
  host: "0.0.0.0"
  port: 8080
  
  # Authentication
  auth:
    enabled: true
    session_timeout: "24h"
    admin_users: ["admin"]
  
  # Video processing
  thumbnails:
    enabled: true
    cache_dir: "./thumbnails"
    quality: "medium"
    
  # Performance settings
  max_concurrent_streams: 10
  cache_size: "1GB"
  
  # Feature flags
  features:
    real_time_updates: true
    bulk_operations: true
    mobile_optimized: true

maintenance:
  quota: "50GB"
  dry_run: true
```

## User Experience Requirements

### Interface Design
- **UX-1.1**: Clean, intuitive interface following modern web design principles
- **UX-1.2**: Consistent visual hierarchy and navigation
- **UX-1.3**: Dark/light theme support
- **UX-1.4**: Keyboard navigation support
- **UX-1.5**: Touch-friendly controls for mobile devices

### User Workflows

#### Primary Workflow: Find and View Recent Footage
1. User opens web application
2. Dashboard shows cameras with recent activity
3. User clicks on camera or date range
4. Grid view displays available recordings with thumbnails
5. User clicks on recording to play in embedded player
6. User navigates using timeline controls

#### Secondary Workflow: Search Historical Footage
1. User accesses search interface
2. Selects camera, date range, and time filters
3. Results display in chronological order
4. User previews using thumbnail hover
5. User plays selected recordings

### Accessibility Requirements
- **UX-2.1**: Screen reader compatibility
- **UX-2.2**: High contrast mode support
- **UX-2.3**: Keyboard-only navigation
- **UX-2.4**: Subtitle support for videos (if available)
- **UX-2.5**: Configurable font sizes

## Success Metrics

### User Adoption
- **M-1.1**: 80% of existing ReoPal users adopt web viewer within 6 months
- **M-1.2**: Average session duration > 10 minutes
- **M-1.3**: Users accessing from mobile devices > 30%

### Performance Metrics
- **M-2.1**: 99.9% uptime
- **M-2.2**: Video playback success rate > 95%
- **M-2.3**: Search query response time < 1 second
- **M-2.4**: Zero security incidents

### Business Metrics
- **M-3.1**: Reduced support tickets related to video access
- **M-3.2**: Increased user satisfaction scores
- **M-3.3**: Faster incident response times for security events

## Implementation Plan

### Phase 1: Core Infrastructure (Weeks 1-4)
- Set up Rust web server with basic API endpoints
- Database integration and video file access
- Basic authentication system
- Simple video listing and metadata API

### Phase 2: Video Playback (Weeks 5-8)
- Video streaming infrastructure
- HTML5 player integration
- Basic search and filtering
- Thumbnail generation system

### Phase 3: Advanced Features (Weeks 9-12)
- Timeline visualization
- Real-time updates via WebSockets
- Mobile-responsive design
- Performance optimization

### Phase 4: Polish and Launch (Weeks 13-16)
- Security hardening
- Accessibility improvements
- Documentation and testing
- Deployment and monitoring setup

## Dependencies and Constraints

### Technical Dependencies
- Existing ReoPal CLI infrastructure
- SQLite database schema compatibility
- Video file format support (MP4)
- Modern web browser compatibility

### Resource Constraints
- Development team availability
- Server hardware requirements
- Network bandwidth for video streaming
- Storage requirements for thumbnails

### External Dependencies
- Web framework selection and learning curve
- Frontend framework and component libraries
- Authentication provider integration
- Monitoring and analytics tools

## Risks and Mitigation

### Technical Risks
- **Risk**: Video streaming performance issues
  - **Mitigation**: Implement progressive loading and quality adaptation
- **Risk**: Database performance with large video catalogs
  - **Mitigation**: Implement pagination, indexing, and caching strategies
- **Risk**: Security vulnerabilities in web interface
  - **Mitigation**: Regular security audits and following web security best practices

### Business Risks
- **Risk**: Low user adoption
  - **Mitigation**: User testing and feedback integration during development
- **Risk**: Increased infrastructure costs
  - **Mitigation**: Implement efficient caching and CDN strategies

## Future Enhancements

### Version 2.0 Considerations
- Mobile applications (iOS/Android)
- Advanced video analytics (motion detection, object recognition)
- Multi-user management with role-based access
- Integration with cloud storage providers
- Live streaming capabilities
- Email/SMS alerts for specific events

### Scalability Considerations
- Microservices architecture for larger deployments
- Database sharding for massive video catalogs
- Load balancing for high-traffic scenarios
- Cloud deployment options

## Conclusion

The ReoPal Web Viewer represents a natural evolution of the existing CLI tool, transforming it into a comprehensive video management platform. By leveraging the existing infrastructure and adding a modern web interface, this feature will significantly improve user accessibility and system usability while maintaining the reliability and performance of the current system.

The phased implementation approach ensures manageable development while delivering value incrementally. The focus on security, performance, and user experience will ensure the web viewer meets the needs of both technical and non-technical users. 