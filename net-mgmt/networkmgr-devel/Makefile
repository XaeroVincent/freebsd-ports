PORTNAME=	networkmgr
DISTVERSION=	7.0
PKGNAMESUFFIX=	-devel
CATEGORIES=	net-mgmt

MAINTAINER=	ericbsd@FreeBSD.org
COMMENT=	FreeBSD/GhostBSD network connection manager
WWW=		https://github.com/GhostBSD/networkmgr

LICENSE=	BSD3CLAUSE
LICENSE_FILE=	${WRKSRC}/LICENSE

BUILD_DEPENDS=  gtk-update-icon-cache:graphics/gtk-update-icon-cache \
		${PYTHON_PKGNAMEPREFIX}python-distutils-extra>=2.15:devel/py-python-distutils-extra@${PY_FLAVOR}
RUN_DEPENDS=	${LOCALBASE}/share/icons/hicolor/index.theme:misc/hicolor-icon-theme

USES=		gettext-tools gnome python shebangfix sudo
USE_GITHUB=	yes
GH_ACCOUNT=	GhostBSD
GH_TAGNAME=	c7768b7c7dad21318807fd823936c9a2888c3766
USE_GNOME=	pygobject3 intltool
USE_PYTHON=	distutils noflavors
SHEBANG_GLOB=	*.py

NO_ARCH=	yes

.include <bsd.port.mk>
