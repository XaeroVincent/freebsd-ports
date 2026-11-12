PORTNAME=	update-station
PORTVERSION=	6.7
CATEGORIES=	ports-mgmt
MASTER_SITES=	https://github.com/GhostBSD/${PORTNAME}/archive/

MAINTAINER=	ericturgeon@ghostbsd.com
COMMENT=	Tool to update FreeBSD/GhostBSD software and system.
LICENSE =	BSD

BUILD_DEPENDS=	${PYTHON_PKGNAMEPREFIX}python-distutils-extra>=2.15:devel/py-python-distutils-extra@${PY_FLAVOR}

RUN_DEPENDS=	sudo:security/sudo \
		pkg:ports-mgmt/pkg \
		distro:sysutils/py-distro@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}bectl>=0.1:sysutils/py-bectl@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}requests>=0.1:www/py-requests@${PY_FLAVOR} \
		${PYTHON_PKGNAMEPREFIX}psutil>=0:sysutils/py-psutil@${PY_FLAVOR}

USE_GITHUB=	yes
GH_ACCOUNT=	GhostBSD

USES =		python:3.9+ gnome gettext-tools
USE_PYTHON=	distutils
USE_GNOME=	pygobject3 intltool 

SUB_FILES=	pkg-install

.include <bsd.port.mk>
