# This file is part of mlnx-ofed. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT. No part of mlnx-ofed, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
# Copyright © 2016-2018 The developers of mlnx-ofed. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/mlnx-ofed/master/COPYRIGHT.


compile_library_name='libibverbs'

compile_library()
{
	compile_autoreconf()
	{
		cd "$rootOutputFolderPath" 1>/dev/null 2>/dev/null

			autoreconf -ivf

		cd - 1>/dev/null 2>/dev/null
	}

	compile_configure()
	{
		cd "$rootOutputFolderPath" 1>/dev/null 2>/dev/null
			
			./configure --prefix=/usr --host="$configureHost" --disable-shared --enable-static --disable-dependency-tracking --disable-silent-rules --enable-fast-install \
				--without-resolve-neigh

		cd - 1>/dev/null 2>/dev/null
	}

	compile_make()
	{
		cd "$rootOutputFolderPath" 1>/dev/null 2>/dev/null

			make -j "$numberOfMakeJobs" 1>&2
			make -j "$numberOfMakeJobs" install DESTDIR="$rootOutputFolderPath"/DESTDIR 1>&2

		cd - 1>/dev/null 2>/dev/null
	}
	
	compile_eradicate_libtools_la_archive_files()
	{
		set +f
			find "$rootOutputFolderPath"/DESTDIR -type f -name '*.la' | xargs rm -rf
		set -f
	}
	
	compile_autoreconf
	
	compile_configure
	
	compile_make
	
	compile_eradicate_libtools_la_archive_files
}

cargo_key_value_pairs()
{
	# compile.conf.d, bindgen-wrapper.conf.d, tools/bindgen-wrapper and lib/$compile_library_name are automatically added.
	cargo_key_value_pairs_link_lib 'static-nobundle' 'libibverbs'
	
	# Search path
	cargo_key_value_pairs_search 'native' "$OUT_DIR"/root/DESTDIR/usr/lib
	
	# Not used by us, but used by downstream ucx-sys crate's build.
	cargo_key_value_pairs_other 'root' "$OUT_DIR"/root/DESTDIR/usr
	cargo_key_value_pairs_other 'include' "$OUT_DIR"/root/DESTDIR/usr/include
	cargo_key_value_pairs_other 'libdir' "$OUT_DIR"/root/DESTDIR/usr/lib
}
